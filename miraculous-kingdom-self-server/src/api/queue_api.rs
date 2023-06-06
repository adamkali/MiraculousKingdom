
use crate::data_types::{
    common::*,
    engine::*,
    queue::{Queue, QueueResonse, QueueItem},
};
use axum::{
    extract::Path,
    Extension,
    Json,
    extract::State
};
use mongodb::{bson::doc, Database};
use std::sync::{Arc, Mutex};
use crate::data_types::common::Reward;


pub mod queue_routes {
    pub use super::get_queue;
    pub use super::take_turn;
    pub use super::set_season;
    pub use super::set_queue;
    pub use super::roll;
}


#[utoipa::path(
    get,
    path = "/api/queue",
    responses((
        status = 200,
        description = "Found Queue from database",
        body = QueueDetailedResponse
    ))
)]
pub async fn get_queue(
    Extension(_mongo): Extension<Database>,
    State(queue): State<Arc<Mutex<Queue>>>,    
) -> Json<QueueDetailedResponse> {
    let queue: DetailedResponse<QueueResonse> 
        = DetailedResponse::new(
            queue.lock().unwrap().as_response()
        );

    Json(queue)
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
    State(queue): State<Arc<Mutex<Queue>>>,    
    Path(pass): Path<String>
) -> Json<QueueDetailedResponse> {
    let mut queue_dr: DetailedResponse<Queue> = DetailedResponse::new(
        queue.lock().unwrap().clone()
    );
    let mut queue_res_dr: DetailedResponse<QueueResonse> 
        = DetailedResponse::new(
            queue_dr.data.as_response()
        );

    
    let mut repo = Repository::<Queue>::new(&mongo, "queues");

    // get the queue from the database
    queue_dr.run(|a| {
       repo.get_by_document(
            a,
            doc! { "game": pass.clone() } 
           ) 
    }).await;


    queue_res_dr.data = queue_dr.data.as_response();
    queue_res_dr.success = queue_dr.success;
    Json(queue_res_dr)
}

#[utoipa::path(
    post,
    path = "/api/queue/turn",
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
)]
pub async fn take_turn(
    Extension(mongo): Extension<Database>,
    State(mut queue): State<Arc<Mutex<Queue>>>,
    Json(turn): Json<TurnRequest>
) -> Json<DetailedResponse<QueueResonse>> {
    let mut state = queue.lock().unwrap().clone();
    let mut resp: DetailedResponse<QueueResonse> = DetailedResponse::new(
        queue.lock().unwrap().as_response()
    );
    let mut game_response: DetailedResponse<Game> = DetailedResponse::new(Game::new());
    let mut game_repo = Repository::<Game>::new(&mongo, "games");

    let item = QueueItem { 
        queue_ability: turn.clone().ability,
        queue_char: turn.clone().character,
        queue_initiative: turn.initiatve as i8,
        queue_roll: None
    };

    // get the game from the database with the same pass
    game_response.run(|a| {
    game_repo.get_by_document(
        a,
        doc! { "pass": state.game.clone() }
    )
    }).await;

    // add the item to the queue
    state.push_queue_item(item);

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

    queue = Arc::new(Mutex::new(state.clone()));
    resp.data = state.as_response();

    // return the queue response
    Json(resp)
}

#[utoipa::path(
    post,
    path = "/api/queue/season",
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
)]
pub async fn set_season(
    Extension(_mongo): Extension<Database>,
    State(mut queue): State<Arc<Mutex<Queue>>>,
    Json(season): Json<SeasonResponse>
) -> Json<DetailedResponse<QueueResonse>> {
    let mut state = queue.lock().unwrap().clone();

    // create a DetailedResponse for the QueueResponse
    let mut resp: DetailedResponse<QueueResonse> = DetailedResponse::new(
        state.as_response()
    );
    
    // set the Season 
    state.season = season.clone();

    // return the queue response
    queue = Arc::new(Mutex::new(state.clone()));
    resp.data = state.as_response();

    // return the queue response
    Json(resp)
}


#[utoipa::path(
    post,
    path = "/api/queue/roll",
    responses((
        status = 200, 
        description = "Roll Added the Queue",
        body = RollDetailedResponse 
    ),
    (
        status = 400, 
        description = "Bad Request: Invalid Roll",
        body = RollDetailedResponse 
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = RollDetailedResponse
    )),
    request_body = RollRequest,
)]
pub async fn roll(
    Extension(mongo): Extension<Database>,
    State(mut queue): State<Arc<Mutex<Queue>>>,
    Json(roll): Json<RollRequest>
) -> Json<RollDetailedResponse> {

    let mut state = queue.lock().unwrap().clone();
    let mut resp: DetailedResponse<RollResponse>
        = DetailedResponse::new(RollResponse::default());

    let mut repo = Repository::<Queue>::new(&mongo, "queues");

    let queue_item = state.queue.iter().find(|a| {
        a.queue_char.secret == roll.owner.clone()
    });
    
    match queue_item {
        Some(item) => {
            let mut item = item.clone();
            item.queue_roll = Some(roll.clone());
            // update the queue with the new item

            state.queue.iter_mut().for_each(|a| {
                if a.queue_char.secret == roll.owner {
                    *a = item.clone();
                }
            });
            
            resp.run(|mut a| {
                async {
                    let result = item.roll().await;
                    match result {
                        Ok(r) => {
                            a.data.rolls = r;
                        },
                        Err(e) => {
                            a.success = Progress::Failing(e);
                        }
                    }
                    a
                }
            }).await;

            resp.run(|mut a| {
                async {
                    a.data.rolls.sort_by(|a, b| {
                        b.roll_value.partial_cmp(&a.roll_value).unwrap()
                    });
                    a
                }
            }).await;

            resp.run(|mut a| {
                async {
                    let owner = a.data.rolls.iter().find(|a| a.roll_character.secret == roll.owner).unwrap().clone();
                    a.data.rolls.retain(|a| a.roll_character.secret != roll.owner);
                    a.data.rolls.insert(0, owner);
                    a
                }
            }).await;
            
            for reward in item.queue_ability.ability_rewards {
                 reward.grant_reward(&mut item.queue_char).unwrap();
            } 

            resp.data.roll_winner = item.queue_char;
        },
        None => {
            resp.success = Progress::Failing(APIError::new(
                axum::http::StatusCode::BAD_REQUEST,
                "The roll owner is not in the queue".to_string()
            ));
        }
    }

    // remove the turn from the queue
    // use queue_item and remove it from the queue.queue 
    state.queue
        .iter()
        .position(|a| 
            a.queue_char.secret == roll.owner
        )
        .map(|a| state.queue.remove(a));

    // update the queue in the database
    let mut state_dr = DetailedResponse::new(
        state.clone()
    );

    state_dr.run(|a| {
        repo.update_one(
            doc! { "game": a.clone().data.game },
            doc! { "$set": { "queue": mongodb::bson::to_bson(&a.data.queue).unwrap() } },
            a
            )
    }).await;

    // now set the queue to the new queue
    queue = Arc::new(Mutex::new(state_dr.data));

    Json(resp)
}
