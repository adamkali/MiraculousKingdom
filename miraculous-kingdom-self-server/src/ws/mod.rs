pub mod structs;
pub mod commands;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use axum::extract::ws::{WebSocket, Message};
use futures::{sink::SinkExt, stream::StreamExt};
use std::collections::HashSet;
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use structs::{WSQueue, Episode};

pub async fn queue_socket(stream: WebSocket, queue: Arc<Mutex<WSQueue>>) {
    let (mut local_sender, mut local_reciever) = stream.split();

    // loop to recieve messages
    while let Some(Ok(Message::Text(character_secret))) = local_reciever.next().await {
        // set the character character_secret
        let character_secret = character_secret.clone();

        // get queue
        let _ = queue
            .lock()
            .unwrap()
            .queue_readiness
            .insert(character_secret, true);
    }

    let mut global_reciever = queue.lock().unwrap().global_broabcast.subscribe();
    let mut global_sender = queue.lock().unwrap().global_broabcast.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(str) = global_reciever.recv().await {
            if *queue.lock().unwrap().is_characers_ready() {
                queue
                    .lock()
                    .unwrap()
                    .queue_readiness
                    .values_mut()
                    .for_each(|v| *v = false);

                match queue.lock().unwrap().queue_state {
                    Episode::NONE => {
                        queue.lock().unwrap().queue_state = Episode::ABILITYCHOOSE;
                    },
                    Episode::ABILITYCHOOSE => {
                        queue.lock().unwrap().queue_state = Episode::TARGETCHOICE;
                    },
                    Episode::TARGETCHOICE => {
                        queue.lock().unwrap().queue_state = Episode::ROLLRESULT;
                        
                    },
                    Episode::ROLLRESULT => {
                        if queue.lock().unwrap().queue_season.event_length == 0 {
                            // here is where we will re roll the season
                            todo!()
                        } else {
                            queue.lock().unwrap().queue_season.event_length -= 1;
                        }
                        queue.lock().unwrap().queue_state = Episode::ABILITYCHOOSE;
                    }
                }
                let ability_choose = serde_json::to_string(&queue
                                                              .lock()
                                                              .unwrap()
                                                              .queue_state
                                                          ).unwrap();
                if global_sender.send(ability_choose).is_err() {
                    break;
                }
            }
        }
    });

    let mut recvieve_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = local_reciever.next().await {
            let mut queue = queue.lock().unwrap(); 

            let mut message = serde_json::from_str::<>(&text).unwrap();
        }
    });
}

