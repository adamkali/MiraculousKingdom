pub mod structs;

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
        SeasonResponse,
    },
    common::{
        Repository,
        DetailedResponse,
    }
};
use futures::{sink::SinkExt, stream::StreamExt};

pub async fn queue_socket(stream: WebSocket, Extension(mongo): Extension<Database>, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_receiver) = stream.split();

    let mut character_secret_local = String::new();

    let queue_clone = Arc::clone(&queue);

    let mut global_receiver_send_task = queue_clone.lock().await.global_broabcast.subscribe();
    let global_sender = queue_clone.lock().await.global_broabcast.clone();
    let global_sender_send_task = global_sender.clone();

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

    let message = serde_json::to_string(&queue_clone.lock().await.queue_state).unwrap();

    match queue_clone.lock().await.global_broabcast.send(message.clone()) {
        Ok(_) => {
            println!("Sent to global {}", message);
        },
        Err(_) => {
            println!("Error sending state to global");
            println!("Queue State: {}", serde_json::to_string_pretty(
                &queue_clone.lock().await.queue_state
            ).unwrap());
        },
    }

    let are_all_ready = queue_clone.lock().await.is_characers_ready();
    println!("Are all ready: {}", are_all_ready);
    if !are_all_ready {
        match global_sender_send_task.send("WAITING".to_string()) {
            Ok(_) => {
                println!("Sent to global WAITING");
            },
            Err(_) => {
                println!("Error sending WAITING to global");
            },
        }

        let is_ready = queue_clone.lock().await.generate_is_ready().await;
        match is_ready {
            Ok(ready) => {
                match global_sender_send_task.send(ready) {
                    Ok(_) => {
                        println!("Sent to global {}", message);
                    },
                    Err(_) => {
                        println!("Error sending is_ready to global");
                    },
                }
            },
            Err(_) => {
                println!("Error sending to global");
            },
        }

    }

    println!("Queue Turn: {}", serde_json::to_string(&queue_clone.lock().await.queue_turn).unwrap());


    let mut send_task = tokio::spawn(async move {
        // broadcast to global the queue state
        loop {
            tracing::debug!("Sending queue state");
            let _ = global_sender_send_task.send("hi".to_string()).unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

    });
    
    let mut recv_task = tokio::spawn(async move {
        // spawn a local broadcaster for the task to communicate 
        while let Some(Ok(Message::Text(text))) = local_receiver.next().await {
            let message = serde_json::from_str::<WSRequest>(&text).unwrap();
            let mut queue_mut = queue.lock().await;
            
            if queue_mut.is_characers_ready() {
                if queue_mut.queue_season.is_none() {
                    let mut seasons_response = DetailedResponse::new( Vec::<Season>::new());
                    let mut response: DetailedResponse<SeasonResponse> = DetailedResponse::new(Season::new().as_response());
                    let mut repository = Repository::<Season>::new(&mongo.clone(), "seasons");
                    seasons_response
                        .run(|a| repository.get_all(a))
                        .await;
                    seasons_response.data.choose(&mut rand::thread_rng()).unwrap();
                    response.success = seasons_response.success;
                    global_sender.send(serde_json::to_string(&response).unwrap()).unwrap();
                }

                match message {
                    WSRequest::READYTOSTART(a) => { 
                        println!("Ready to start");
                        match queue_mut.receive_request(&a).await {
                            Ok(_) => {
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
                        // TODO: need to broadcast to all clients that if they are any targets that need to
                        // be rolled 
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
                    local_sender.send(Message::Text(queue_mut.generate_is_ready().await.unwrap())).await.unwrap();
                }

            } else {
                println!("WAITING");
                local_sender.send(Message::Text("WAITING".to_string())).await.unwrap();
                local_sender.send(Message::Text(queue_mut.generate_is_ready().await.unwrap())).await.unwrap();
            }

        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

