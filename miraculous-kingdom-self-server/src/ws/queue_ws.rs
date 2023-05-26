
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
    use futures::{sink::SinkExt, stream::StreamExt};

    use crate::data_types::websockets::*;


    pub async fn queue_ws_handler(
        ws: WebSocketUpgrade,
        Extension(mongo): Extension<mongodb::Database>,
        State(queue): State<Arc<Mutex<Queue>>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>, 
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_queue_message(socket, addr, queue))
    }

    pub async fn handle_queue_message(
        mut socket: WebSocket,
        who: SocketAddr,
        queue: Arc<Mutex<Queue>>,
        mongo: mongodb::Database,
    ) {
        {
            let mut state: Queue = queue.lock().unwrap().clone();
            state.sockets.push(who);
        }

        // ping the user 
        let (mut tx, mut rx) = socket.split();
        tx.send(Message::Text(format!("Hello, {}", who))).await;

        while let Some(Ok(message)) = rx.next().await {
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
                        handle_start_message(who, game, queue, mongo).await
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
    // by sending the socket "OK"
    async fn handle_start_message(
        who: SocketAddr,
        game: String,
        queue: Arc<Mutex<Queue>>,
        mongo: mongodb::Database
    ) {
        let mut game_response = DetailedResponse::new(QueueModel::new());

        let mut game_repo = Repository::<Queue>::new(&mongo, "queues");

        game_response
            .run(|a| game_repo.get_by_document(a, doc! { "generated_pass": pass.clone() }))
            .await
            .absorb(&mut char_response);
    }

}
