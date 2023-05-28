
use crate::data_types::{
    characters::{Character, CharacterResponse, Class, NewCharacter, AbilityModel},
    common::DetailedResponse,
    common::MKModel,
    common::Progress,
    common::Repository,
    common::TurnRequest,
    engine::*,
    game_to_info, games_to_info,
    queue::{Queue, QueueResonse, QueueItem}, 
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use mongodb::{bson::doc, Database};

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
pub async fn get_game_queue(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
) -> Json<DetailedResponse<QueueResonse>> {
    let mut queue: DetailedResponse<Queue> = DetailedResponse::new(Queue::new());
    // create a DetailedResponse for the QueueResponse
    let mut resp: DetailedResponse<QueueResonse> = DetailedResponse::new(
        queue.clone().data.as_response()
    );

    let mut repo = Repository::<Queue>::new(&mongo, "queues");

    // get the queue from the database
    queue.run(|a| {
       repo.get_by_document(
            a,
            doc! { "game": pass },
           ) 
    }).await;

    resp.data = queue.data.as_response();
    resp.success = queue.success;
    // return the queue response
    Json(resp)
}

#[utoipa::path(
    post,
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
    ),
    request_body = TurnRequest,
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
    queue.data.status = queue.data.queue.len() 
                    == game_response.data.game_chars.len();

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
    get,
    path = "/api/queue/roll",
    responses((
        status = 200, 
        description = "Listed classes from database", 
        body = SeasonDetailedResponse 
    ),
    (
        status = 400, 
        description = "Bad Request: id", 
        body = SeasonDetailedResponse
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = SeasonDetailedResponse 
    ))
)]
pub async fn roll(Extension(mongo): Extension<Database>) -> Json<DetailedResponse<SeasonResponse>> {
    let mut seasons_response: DetailedResponse<Vec<Season>> =
        DetailedResponse::new(Vec::<Season>::new());
    let mut response: DetailedResponse<Season> = DetailedResponse::new(Season {
        event_id: ObjectId::new(),
        event_name: String::new(),
        event_desc: String::new(),
        event_length: 1,
        event_reward: RewardTypes::None,
    });

    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    seasons_response
        .run(|a| repository.get_all(a))
        .await
        .absorb(&mut response);
    match seasons_response.data.choose(&mut rand::thread_rng()) {
        Some(a) => {
            response.data = a.clone();
        }
        None => {
            response.set_code(Some(APIError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Counld not get a random event".to_string(),
            )));
        }
    }
    let mut res: DetailedResponse<SeasonResponse> =
        DetailedResponse::new(response.data.as_response());
    res.absorb(&mut response);
    Json(res)
}
