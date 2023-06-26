pub mod structs;
pub mod commands;

use std::sync::{Arc};
use tokio::sync::{Mutex};
use std::collections::HashMap;
use axum::extract::ws::{WebSocket, Message};
use futures::{stream::StreamExt};
use structs::*;
use crate::data_types::common::{MKModel};
use axum::extract::Extension;
use mongodb::Database;
use rand::seq::SliceRandom;

pub async fn queue_socket(stream: WebSocket, Extension(mongo): Extension<Database>, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_receiver) = stream.split();

    let mut character_secret_local = String::new();

    // loop to receive messages
    while let Some(Ok(Message::Text(character_secret))) = local_receiver.next().await {
        // set the character secret
        character_secret_local = character_secret.clone();

        // get queue
        let mut queue_turn = queue.lock().await.queue_turn.clone();
        queue_turn.insert(
            character_secret.clone(),
            Turn {
                turn_ability: crate::data_types::characters::AbilityModel::new().as_response(),
                turn_characters: HashMap::new(),
                turn_ready: false,
            },
        );
    }

    let queue_clone = Arc::clone(&queue);

    let mut global_receiver = queue_clone.lock().await.global_broabcast.subscribe();
    let global_sender = queue_clone.lock().await.global_broabcast.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(_) = global_receiver.recv().await {
            let mut queue_mut = queue_clone.lock().await;
            if queue_mut.is_characers_ready() {
                let mut cloned_queue = queue_mut.clone();
                match cloned_queue.queue_state {
                    Episode::NONE => {
                        // roll the season b
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
                        // send that to the global broadcast
                    }
                    Episode::ABILITYCHOOSE => {
                        cloned_queue.queue_state = Episode::TARGETCHOICE;
                    }
                    Episode::TARGETCHOICE => {
                        cloned_queue.queue_state = Episode::ROLLRESULT;
                    }
                    Episode::ROLLRESULT => {
                        // before we do anything, bradcast the result of the rolls 
                        // to the global broadcast
                        // and then change the state to resolution

                    }
                    Episode::RESOLUTION => {
                        match cloned_queue.queue_season.as_mut() {
                            Some(season) => {
                                season.event_length = season.event_length - 1;
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
                    // send that to the global broadcast
                }
                let _ = global_sender.send(serde_json::to_string(&cloned_queue.queue_state).unwrap());
                let state = serde_json::to_string(&cloned_queue.queue_state).unwrap();
                if global_sender.send(state).is_err() {
                    // let everyone know that something went wrong
                    todo!()
                }
                *queue_mut = cloned_queue; // Replace the original queue with the cloned data
            } else {
                let _ = global_sender.send("NOT READY".to_string());
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = local_receiver.next().await {
            let message = serde_json::from_str::<WSRequest>(&text).unwrap();
            let mut queue_mut = queue.lock().await;
            match message {
                WSRequest::ABILITYREQUEST(a) => {
                    queue_mut.receive_request(&a);
                }
                WSRequest::CHARACTERREQUEST(target) => {
                    queue_mut.receive_request(&target);
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
                            .find(|x| &x.secret == &result.clone().secret)
                            .unwrap()
                            .clone()
                    );
                    queue_mut.receive_request(&result);
                }
            }
            // set the person character turn to be true
            queue_mut.queue_turn.iter_mut().for_each(|x| {
                if *x.0 == character_secret_local {
                    x.1.turn_ready = true;
                }
            });
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}


