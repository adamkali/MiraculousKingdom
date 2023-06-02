
use crate::data_types::{
    common::*,
    engine::*,
    queue::{Queue, QueueResonse, QueueItem},
};
use axum::{extract::Path, Extension, Json};
use mongodb::{bson::doc, Database};

pub mod queue_routes {
    pub use super::get_queue;
    pub use super::take_turn;
    pub use super::set_season;
}


#[utoipa::path(
    get,
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
        ("pass" = String, Path, description = "Password for entering the game.")
    )
)]
pub async fn get_queue(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
) -> Json<QueueDetailedResponse> {
    let mut queue_model: DetailedResponse<Queue> = DetailedResponse::new(Queue::new());
    let mut queue: DetailedResponse<QueueResonse> = DetailedResponse::new(Queue::new().as_response());
    let mut repo = Repository::<Queue>::new(&mongo, "queues");
    // get the queue from the database
    queue_model.run(|a| {
       repo.get_by_document(
            a,
            doc! { "game": pass },
           ) 
    }).await;

    // set the queue 
    queue.data = queue_model.clone().data.as_response();
    queue.success = queue_model.success;

    Json(queue)
}

#[utoipa::path(
    post,
    path = "/api/queue/turn/{pass}",
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
    request_body = TurnRequest,
    params(
        ("pass" = String, Path, description = "Password for entering the game.")
    )
)]
pub async fn take_turn(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
    Json(turn): Json<TurnRequest>
) -> Json<DetailedResponse<QueueResonse>> {
    let mut queue: DetailedResponse<Queue> = DetailedResponse::new(Queue::new());
    // create a DetailedResponse for the QueueResponse
    let mut resp: DetailedResponse<QueueResonse> = DetailedResponse::new(
        queue.clone().data.as_response()
    );
    let mut game_response: DetailedResponse<Game> = DetailedResponse::new(Game::new());
    let mut game_repo = Repository::<Game>::new(&mongo, "games");

    let mut item = QueueItem { 
        queue_ability: turn.clone().ability,
        queue_char: turn.clone().character,
        queue_initiative: turn.initiatve as i8
    };

    let mut repo = Repository::<Queue>::new(&mongo, "queues");

    // get the queue from the database
    queue.run(|a| {
       repo.get_by_document(
            a,
            doc! { "game": pass.clone() } 
           ) 
    }).await;

    // get the game from the database with the same pass
    game_response.run(|a| {
    game_repo.get_by_document(
        a,
        doc! { "pass": pass.clone() }
    )
    }).await;

    // add the item to the queue
    queue.data.push_queue_item(item);

    // update the queue in the database
    queue.run(|a| {
            repo.update_one(
                doc! { "game" : a.clone().data.game },
                doc! { "$set": {
                    "queue": mongodb::bson::to_bson(
                        &a.data.queue
                    ).unwrap()
                }},
                a
            )
    }).await;

    turn.character.char_hand.iter().for_each(|a| {
        if a.ability_name == turn.ability.ability_name {
            turn.clone().character.char_hand.retain(|b| {
                b.ability_name != turn.ability.ability_name
            });

            turn.character.clone().char_discard.push(
                turn.ability.clone()
            );
        }
    });

    // use the run method to get the game from the Database
    // and update the game with the new character
    game_response.run(|a| {
        game_repo.update_one(
            doc! { "pass": pass.clone() },
            doc! { "$set": {
                "game_chars": mongodb::bson::to_bson(
                    &a.data.game_chars
                ).unwrap()
            }},
            a
        )
    }).await;

    // return the queue response
    resp.data = queue.data.as_response();
    resp.success = queue.success;

    // return the queue response
    Json(resp)
}

#[utoipa::path(
    post,
    path = "/api/queue/season/{pass}",
    responses((
        status = 200, 
        description = "Listed classes from database", 
        body = QueueDetailedResponse 
    ),
    (
        status = 400, 
        description = "Bad Request: id", 
        body = QueueDetailedResponse
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = QueueDetailedResponse 
    )),
    request_body = SeasonResponse,
    params(
        ("pass" = String, Path, description = "Password for entering the game.")
    )
)]
pub async fn set_season(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
    Json(season): Json<SeasonResponse>
) -> Json<DetailedResponse<QueueResonse>> {
    let mut queue: DetailedResponse<Queue> = DetailedResponse::new(Queue::new());
    // create a DetailedResponse for the QueueResponse
    let mut resp: DetailedResponse<QueueResonse> = DetailedResponse::new(
        queue.clone().data.as_response()
    );
    
    let mut repo = Repository::<Queue>::new(&mongo, "queues");

    // set the Season 
    queue.data.season = season.clone();

    // update the queue
    queue.run(|a| {
        repo.update_one(
            doc! { "game": pass.clone() },
            doc! { "$set": { "season": mongodb::bson::to_bson(&a.data.season).unwrap() } },
            a
        ) 
    })
    .await;

    // return the queue response
    resp.data = queue.data.as_response();
    resp.success = queue.success;

    // return the queue response
    Json(resp)
}
