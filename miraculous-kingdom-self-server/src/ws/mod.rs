pub mod structs;
pub mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use axum::extract::ws::{WebSocket, Message};
use structs::*;
use crate::data_types::common::MKModel;
use axum::extract::Extension;
use mongodb::Database;
use rand::seq::SliceRandom;

use crate::data_types::{
    engine::{
        Season,
        SeasonEnum,
        SeasonResponse,
        RewardTypes
    },
    common::{
        Repository,
        DetailedResponse,
    }
};

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn queue_socket(stream: WebSocket, Extension(mongo): Extension<Database>, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_receiver) = stream.split();

    let mut character_secret_local = String::new();

    // loop to receive messages
    while let Some(Ok(Message::Text(character_secret))) = local_receiver.next().await {
        // set the character secret
        character_secret_local = character_secret.clone();

        // get queue
        match queue.lock().await.queue_turn.entry(character_secret_local.clone()) {
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
    }

    let queue_clone = Arc::clone(&queue);

    println!("Queue Turn: {}", serde_json::to_string(&queue_clone.lock().await.queue_turn).unwrap());

    let mut global_receiver = queue_clone.lock().await.global_broabcast.subscribe();
    let global_sender = queue_clone.lock().await.global_broabcast.clone();

    let mut send_task = tokio::spawn(async move {
        println!("Send Task Started");
        while  global_receiver.recv().await.is_ok() {
            // send the queue to the client
        }
    });
    
    let mut recv_task = tokio::spawn(async move {
        // spawn a local broadcaster for the task to communicate 
        while let Some(Ok(Message::Text(text))) = local_receiver.next().await {
            let message = serde_json::from_str::<WSRequest>(&text).unwrap();
            let mut queue_mut = queue.lock().await;
            if let None = queue_mut.queue_season {
                let mut seasons_response = DetailedResponse::new( Vec::<Season>::new());
                let mut response: DetailedResponse<SeasonResponse> = DetailedResponse::new(Season::new().as_response());

                let mut repository = Repository::<Season>::new(&mongo.clone(), "seasons");

                seasons_response
                    .run(|a| repository.get_all(a))
                    .await;

                println!("{}", serde_json::to_string_pretty(&seasons_response).unwrap());

                seasons_response.data.choose(&mut rand::thread_rng()).unwrap();

                response.success = seasons_response.success;
                global_sender.send(serde_json::to_string(&response).unwrap()).unwrap();
            }
            match message {
                WSRequest::READYTOSTART(a) => { 
                    println!("Ready to start");
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            println!("Ready to start");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                x.1.turn_ready = true;
                            });
                        }
                        Err(e) => {
                            local_sender.send(Message::Text(serde_json::to_string(&e).unwrap())).await.unwrap();
                        }
                    }
                }
                WSRequest::ABILITYREQUEST(a) => {
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            println!("Ability request");
                            let state = serde_json::to_string(&queue_mut.queue_state).unwrap();
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                            local_sender.send(Message::Text(state)).await.unwrap();
                        }
                        Err(_) => {
                            local_sender.send(Message::Text("Not a ABILITYREQUEST request".to_string())).await.unwrap();
                        }
                    }

                }
                WSRequest::CHARACTERREQUEST(a) => {
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            println!("Character request");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                        }
                        Err(_) => {
                            local_sender.send(Message::Text("Not a CHARACTERREQUEST request".to_string())).await.unwrap();
                        }
                    }
                }
                WSRequest::ROLLREQUEST(mut result) => {
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
                    match queue_mut.receive_request(&result).await {
                        Ok(_) => {
                            println!("Roll request");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                        }
                        Err(_) => {
                            // Clear out the result ability and character as the request was not valid
                            result.ability = None;
                            result.character = None;
                            local_sender
                                .send(Message::Text(
                                    "Not a character request or not your turn".to_string(),
                                ))
                                .await
                                .unwrap();
                        }
                    }
                }
            }
            if queue_mut.advance_episode_state(mongo.clone()).await {
                let state = serde_json::to_string(&queue_mut.queue_state).unwrap();
                global_sender.send(state).unwrap();
            } else {
                local_sender.send(Message::Text("WAITING".to_string())).await.unwrap();
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

