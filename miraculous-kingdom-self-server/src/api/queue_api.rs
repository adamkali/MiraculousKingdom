use crate::data_types::{
    common::*,
    engine::*,
};
use axum::{
    extract::Path,
    Extension,
    Json,
    extract::{
        ws::WebSocketUpgrade,
        State,
    },
    response::IntoResponse,
};
use mongodb::{bson::doc, Database};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ws;

pub mod queue_routes {
    pub use super::ws_entyrpoint;
    pub use super::set_queue;
}



#[utoipa::path(
    put,
    path = "/api/queue/{pass}",
    responses((
        status = 200,
        description = "Found Queue from database",
        body = QueueDetailedResponse
    ),
    (
        status = 404,
        description = "Could not find queue from database",
        body = QueueDetailedResponse
    ),
    (
        status = 500,
        description = " Internal error occured",
        body = QueueDetailedResponse 
    )),
    params(
        (
            "pass" = String, 
            Path, 
            description = "Password of the game to be set"
        )
    )
)]
pub async fn set_queue(
    Extension(mongo): Extension<Database>,
    State(queue): State<Arc<Mutex<ws::structs::WSQueue>>>,    
    Path(pass): Path<String>
) -> Json<DetailedResponse<bool>> {
    let mut game_repo = Repository::<Game>::new(&mongo, "games");

    let game_response = 
        DetailedResponse::new(Game::new())
            .run(|a| game_repo.get_by_document(
                 a, doc! { "generated_pass": pass }
            )).await;

    let reference = game_response
        .data
        .game_chars
        .iter()
        .map(|a| a.clone().as_response())
        .collect::<Vec<CharacterResponse>>();

    queue.lock().await.queue = reference;

    
    let mut detailed_response = 
        DetailedResponse::new(false);

    detailed_response.success = game_response.success;
    
    if let crate::data_types::common::Progress::Succeeding = detailed_response.success {
        detailed_response.data = true;
    }

    Json(detailed_response)

}

#[utoipa::path(
    get,
    path = "/api/queue",
    responses((
        status = 200,
        description = "Found Queue from database",
    ),
    (
        status = 404,
        description = "Could not find queue from database",
    ),
    (
        status = 500,
        description = " Internal error occured",
    ))
)]
pub async fn ws_entyrpoint(
    ws: WebSocketUpgrade,
    Extension(mongo): Extension<Database>,
    State(queue): State<Arc<Mutex<ws::structs::WSQueue>>>
) -> impl IntoResponse {
   ws.on_upgrade(|socket| ws::queue_socket(socket, Extension(mongo), queue))
}
