
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

    use crate::data_types::websockets::*


    pub async fn queue_ws_handler(
        ws: WebSocketUpgrade,
        State(queue): State<Queue>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>, 
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| todo!())
    }
}
