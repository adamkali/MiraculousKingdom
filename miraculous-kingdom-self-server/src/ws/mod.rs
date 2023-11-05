//pub mod structs;
//
//use std::sync::Arc;
//use tokio::sync::Mutex;
//use std::collections::HashMap;
//use axum::extract::ws::{WebSocket, Message};
//use structs::*;
//use crate::data_types::common::MKModel;
//use axum::extract::Extension;
//use mongodb::Database;
//use rand::seq::SliceRandom;
//
//use crate::data_types::{
//    engine::{
//        Season,
//        SeasonResponse,
//    },
//    common::{
//        Repository,
//        DetailedResponse,
//    }
//};
//use futures::{sink::SinkExt, stream::StreamExt};

pub async fn queue_socket(stream: WebSocket, Extension(mongo): Extension<Database>, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_receiver) = stream.split();

    let mut character_secret_local = String::new();

    //let queue_clone = Arc::clone(&queue);
    //let cleanup_clone = Arc::clone(&queue);

    //let mut global_receiver_send_task = queue_clone.lock().await.global_broabcast.subscribe();
    //let global_sender = queue_clone.lock().await.global_broabcast.clone();
    //let global_sender_send_task = global_sender.clone();

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
                tracing::info!("Added to queue");
                break;
            },
            std::collections::hash_map::Entry::Occupied(_) => {
                tracing::info!("Not added to queue");
            },
        }
    }

    let char_clone = character_secret_local.clone();
    let conn_clone = mongo.clone();
    let mut send_task = tokio::spawn(async move {
        // broadcast to global the queue state
        loop {
            if queue_clone.lock().await.queue_season.is_none() {
                
                let mut seasons_response = DetailedResponse::new( Vec::<Season>::new());
                let mut response: DetailedResponse<SeasonResponse> = DetailedResponse::new(Season::new().as_response());
                let mut repository = Repository::<Season>::new(&conn_clone, "seasons");

                seasons_response
                    .run(|a| repository.get_all(a))
                    .await;
                response.success = seasons_response.success;
                response.data = seasons_response.data.choose(&mut rand::thread_rng()).unwrap().as_response();
                let ws_response = WSResponse::SEASONRESPONSE(response.data.clone());
                let response_message = match serde_json::to_string_pretty(&ws_response) {
                    Ok(message) => message,
                    Err(_) => break,
                };

                tracing::debug!("Sending out Season Response: {}", response_message.clone());
                queue_clone.lock().await.queue_season = Some(response.data);
                global_sender.send(response_message).unwrap();
            }
        }
    });
    
    let mut recv_task = tokio::spawn(async move {

        while let Some(Ok(Message::Text(text))) = local_receiver.next().await {
            tracing::debug!("Received message {}", text);
            let message = serde_json::from_str::<WSRequest>(&text).unwrap();
            let mut queue_mut = queue.lock().await;

            match message {
                WSRequest::READYTOSTART(a) => { 
                    tracing::debug!("Ready to start");
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            tracing::debug!("processed ready to start");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                            tracing::info!("calling advance turn");
                        }
                        Err(e) => {
                            tracing::warn!("error processing ready to start: {}", e);
                        }
                    }
                }

                WSRequest::ABILITYREQUEST(a) => {
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            tracing::debug!("Ability request");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                        }
                        Err(_) => {
                            tracing::warn!("error processing ability request");
                        }
                    }
                }
                
                WSRequest::CHARACTERREQUEST(a) => {
                    match queue_mut.receive_request(&a).await {
                        Ok(_) => {
                            tracing::debug!("Character request");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                        }
                        Err(_) => {
                            tracing::warn!("error processing character request");
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
                            tracing::debug!("Roll request");
                            queue_mut.queue_turn.iter_mut().for_each(|x| {
                                if *x.0 == character_secret_local.clone() {
                                    x.1.turn_ready = true;
                                }
                            });
                        }
                        Err(_) => {
                            result.ability = None;
                            result.character = None;
                            tracing::debug!("Roll request failed");
                        }
                    }
                }
            }
            if queue_mut.advance_episode_state(mongo.clone()).await {
                let state = queue_mut.queue_state.clone();
                tracing::debug!("advanced episode {:?}", state.clone());
                let ws_response = WSResponse::EPISODERESPONSE(state);
                let ws_response_message = serde_json::to_string(&ws_response).unwrap();
                local_sender.send(Message::Text(ws_response_message)).await.unwrap();
            } else {
                local_sender.send(
                    Message::Text(serde_json::to_string(&WSResponse::WAITING).unwrap())
                ).await.unwrap();
                local_sender.send(
                    Message::Text(queue_mut.generate_is_ready().await.unwrap())
                ).await.unwrap();
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    cleanup_clone.lock().await.remove_player(&char_clone).await;
}

