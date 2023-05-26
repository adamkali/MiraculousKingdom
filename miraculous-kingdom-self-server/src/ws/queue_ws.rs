
pub mod queue {
    use axum::{
        extract::{
            ws::{
                Message,
                WebSocket,
                WebSocketUpgrade,
            },
            State,
            Extension,
        },
        response::IntoResponse,
    };

    use std::sync::{Arc, Mutex};

    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    //allows to extract the IP of connecting user
    use axum::extract::connect_info::ConnectInfo;
    use axum::extract::ws::CloseFrame;
    use std::{net::SocketAddr, path::PathBuf};
    //allows to split the websocket stream into separate TX and RX branches
    use futures::{sink::SinkExt, stream::{StreamExt, SplitSink}};

    use crate::data_types::websockets::*;
    use crate::data_types::common::*;
    use mongodb::bson::doc;


    pub async fn queue_ws_handler(
        ws: WebSocketUpgrade,
        Extension(mongo): Extension<mongodb::Database>,
        State(queue): State<Arc<Mutex<Queue>>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>, 
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_queue_message(
                socket, 
                addr, 
                queue,
                mongo
            ))
    }

    pub async fn handle_queue_message(
        mut socket: WebSocket,
        who: SocketAddr,
        mut queue: Arc<Mutex<Queue>>,
        mut mongo: mongodb::Database,
    ) {
        {
            let mut state: Queue = queue.lock().unwrap().clone();
            state.sockets.push(who);
            queue = Arc::new(Mutex::new(state.clone()));
        }

        // ping the user 
        let (mut tx, mut rx) = socket.split();
        tx.send(Message::Text(format!("Hello, {}", who))).await;

        while let Some(Ok(message)) = rx.next().await {
            let queue_clone = queue.clone();
            let mongo_clone = mongo.clone();
            if let Message::Close(Some(CloseFrame { code, reason })) = message {
                tracing::info!(
                    "client {} closed connection with code {:?} and reason {:?}",
                    who,
                    code,
                    reason
                );
                return;
            } 
            if let Message::Text(queue_message) = message {
                tracing::info!("client {} sent message: {:?}", who, queue_message);

                let command: WebsocketMessage = match serde_json::from_str(&queue_message) {
                   Err(e) => {
                       panic!("{}", e.to_string());
                   },
                   Ok(mess) => { mess }
                };
                
                match command {
                    WebsocketMessage::Start(game) => {
                        tx.send(
                            handle_start_message(game, queue_clone, mongo_clone).await
                        )
                        .await
                        .expect("Failed to send message");
                    },
                    _ => {}
                }


            } else {
                panic!("or nawr");
            }
        }
    }

    // Start Message 
    // need to load the Queue from the database
    // using a QueueModel
    // then ping the user that everything worked out
    // by sending the socket "OK
    async fn handle_start_message(
        game: String,
        queue: Arc<Mutex<Queue>>,
        mongo: mongodb::Database
    ) -> Message {
        let mut game_response = 
            crate::data_types::common::DetailedResponse::new(QueueModel::new());

        let mut game_repo = Repository::<QueueModel>::new(&mongo, "queues");

        game_response
            .run(|a| game_repo.get_by_document(a, doc! { "game": game.clone() }))
            .await;

        // Lock the queue, unwrap it,  and then set the queueModel to the 
        // Queue as_response();
        let queue_lock = queue.lock();
        match queue_lock {
            Ok(mut q) => {
                *q = game_response.data.as_response();
            },
            Err(e) => {
                return Message::Text("Or Nor".to_string());
            }
        }

        // Send the socket the OK message
        Message::Text("OK".to_string())
    }

}
