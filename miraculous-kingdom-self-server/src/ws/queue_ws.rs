
pub mod queue {
    use axum::{
        extract::{
            ws::{
                Message,
                WebSocket,
                WebSocketUpgrade,
            },
            State
        },
        response::IntoResponse,
    };

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
        State(queue): State<Queue>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>, 
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| todo!())
    }

    pub async fn handle_queue_message(
        mut socket: WebSocket,
        who: SocketAddr,
        queue: Queue
    ) {
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



            } else {
                panic!("or nawr");
            }
        }
    }

    // Start Message 
    // need to load the Queue from the database

}
