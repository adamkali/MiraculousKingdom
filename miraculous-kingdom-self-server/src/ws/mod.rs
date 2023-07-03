pub mod structs;
pub mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use axum::extract::ws::{WebSocket, Message};
use futures::stream::StreamExt;
use structs::*;
use crate::data_types::common::MKModel;
use axum::extract::Extension;
use mongodb::Database;
use rand::seq::SliceRandom;
use futures::stream::{SplitSink, SplitStream};

pub async fn queue_socket(stream: WebSocket, Extension(_mongo): Extension<Database>, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_receiver) = stream.split();

    let mut character_secret_local = String::new();

    // loop to receive messages
    while let Some(Ok(Message::Text(character_secret))) = local_receiver.next().await {
        // set the character secret
        character_secret_local = character_secret.clone();

        // get queue
        let mut queue_turn = queue.lock().await.queue_turn.clone();

        match queue_turn.entry(character_secret_local.clone()) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(Turn {
                    turn_ready: false,
                    turn_ability: crate::data_types::characters::Ability::default(),
                    turn_characters: HashMap::new(),
                });
                println!("Added to queue");
                break;
            },
            std::collections::hash_map::Entry::Occupied(_) => {
                println!("Not added to queue");
            },
        }
        

        // print out the queue turn now 
        println!("Queue Turn: {}", serde_json::to_string(&queue_turn).unwrap());
    }

    let queue_clone = Arc::clone(&queue);

    println!("Queue Turn: {}", serde_json::to_string(&queue_clone.lock().await.queue_turn).unwrap());

    let mut global_receiver = queue_clone.lock().await.global_broabcast.subscribe();
    let global_sender = queue_clone.lock().await.global_broabcast.clone();

    let mut send_task = tokio::spawn(async move {
        println!("Send Task Started");
        while let Ok(_) = global_receiver.recv().await {
            println!("starts sending");
            let mut queue_mut = queue_clone.lock().await;
            queue_mut.generate_is_ready().await.unwrap();
            if queue_mut.is_characers_ready() {
                let mut cloned_queue = queue_mut.clone();
                match cloned_queue.queue_state {
                    Episode::NONE => {
                        let season_response_throw = crate::data_types::common::DetailedResponse::new(
                            Vec::<crate::data_types::engine::Season>::new()
                        );
                        cloned_queue.queue_season =  Some(
                            season_response_throw
                                .data
                                .choose(&mut rand::thread_rng())
                                .unwrap()
                                .clone()
                                .as_response()
                            );
                        cloned_queue.queue_state = Episode::ABILITYCHOOSE;
                    }
                    Episode::ABILITYCHOOSE => {
                        cloned_queue.queue_state = Episode::TARGETCHOICE;
                    }
                    Episode::TARGETCHOICE => {
                        cloned_queue.queue_state = Episode::ROLLRESULT;
                    }
                    Episode::ROLLRESULT => {
                        cloned_queue.generate_results().await;
                        cloned_queue.queue_state = Episode::RESOLUTION;
                    }
                    Episode::RESOLUTION => {
                        match cloned_queue.queue_season.as_mut() {
                            Some(season) => {
                                season.event_length -= 1;
                                if season.event_length == 0 {
                                    cloned_queue.queue_season = None;
                                    cloned_queue.queue_state = Episode::NONE;
                                } else {
                                    cloned_queue.queue_state = Episode::ABILITYCHOOSE;
                                }
                            }
                            None => { unreachable!() }
                        }
                    }
                }
                println!("Queue state: {:?}", cloned_queue.queue_state);
                let _ = global_sender.send(serde_json::to_string(&cloned_queue.queue_state).unwrap());
                let state = serde_json::to_string(&cloned_queue.queue_state).unwrap();
                if global_sender.send(state).is_err() {
                    let _ = global_sender.send("NOT READY".to_string());
                }
                *queue_mut = cloned_queue; // Replace the original queue with the cloned data
            }
        }
    });
    
    let character_secret_local_clone = character_secret_local.clone();
    let mut recv_task = tokio::spawn(async move {
        // spawn a local broadcaster for the task to communicate 
        let (local_sender, mut local_receiver) = broadcast::channel(10);
        while let Some(Ok(Message::Text(text))) = local_receiver.next().await {
            let message = serde_json::from_str::<WSRequest>(&text).unwrap();
            let mut queue_mut = queue.lock().await;
            match message {
                WSRequest::READYTOSTART(a) => { 
                    println!("Ready to start");
                    match queue_mut
                        .receive_request(&a)
                        .await {
                            Ok(_) => {
                                println!("Ready to start");
                                queue_mut.queue_turn.iter_mut().for_each(|x| {
                                    x.1.turn_ready = false;
                                });
                                let state = serde_json::to_string(&queue_mut.queue_state).unwrap();
                            }
                            Err(_) => {
                                println!("Not ready to start");
                            }
                        }
                        
                }
                WSRequest::ABILITYREQUEST(a) => {
                    queue_mut.receive_request(&a).await.unwrap();
                    queue_mut.queue_turn.iter_mut().for_each(|x| {
                        if *x.0 == character_secret_local.clone() {
                            x.1.turn_ready = true;
                        }
                    });
                }
                WSRequest::CHARACTERREQUEST(target) => {
                    queue_mut.receive_request(&target).await.unwrap();
                    queue_mut.queue_turn.iter_mut().for_each(|x| {
                        if *x.0 == character_secret_local.clone() {
                            x.1.turn_ready = true;
                        }
                    });
                }
                WSRequest::ROLLREQUEST(mut result) => {
                    // need to pass the result to have the abibility as well
                    // as the character  
                    result.ability = Some(
                        queue_mut
                            .queue_turn
                            .iter()
                            .find(|x| x.0 == &result.clone().get_owner())
                            .unwrap()
                            .1
                            .turn_ability
                            .clone()
                    );
                    result.character = Some(
                        queue_mut
                            .queue
                            .iter()
                            .find(|x| x.secret == result.clone().secret)
                            .unwrap()
                            .clone()
                    );
                    queue_mut.receive_request(&result).await.unwrap();
                    queue_mut.queue_turn.iter_mut().for_each(|x| {
                        if *x.0 == character_secret_local {
                            x.1.turn_ready = true;
                        }
                    });
                }
            }
            // set the person character turn to be true
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}


