use crate::data_types::{
    characters::{Character, CharacterResponse, Class, NewCharacter, AbilityModel},
    common::DetailedResponse,
    common::MKModel,
    common::Progress,
    common::Repository,
    common::TurnRequest,
    engine::*,
    game_to_info, games_to_info,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use mongodb::{bson::doc, Database};

#[utoipa::path(
    get,
    path = "/api/game",
    responses(
        (
            status = 200, 
            description = "Game found",
            body = GamesInfoDetailedResponse,
        ),
        (
            status = 500, 
            description = "Internal Server Error",
            body = GamesInfoDetailedResponse,
        )
    )
)]
pub async fn get_games(
    Extension(mongo): Extension<Database>,
) -> Json<DetailedResponse<Vec<GameInfo>>> {
    let mut response: DetailedResponse<Vec<GameInfo>> =
        DetailedResponse::new(Vec::<GameInfo>::new());
    let mut game_response: DetailedResponse<Vec<Game>> 
        = DetailedResponse::new(Vec::<Game>::new());

    let mut repository = Repository::new(&mongo, "games");
    game_response.run(|a| repository.get_all(a)).await;
    games_to_info(game_response.clone().data, &mut response.data).await.unwrap();

    response.success = game_response.clone().success;

    Json(response)
}

#[utoipa::path(
    get,
    path = "/api/game/{pass}",
    responses((
        status = 200,
        description = "Found class from database",
        body = GameInfoDetailedResponse
    ),
    (
        status = 404,
        description = "Could not find class from database",
        body = GameInfoDetailedResponse
    ),
    (
        status = 500,
        description = " Internal error occured",
        body = GameInfoDetailedResponse
    )),
    params(
        ("pass" = String, Path, description = "Password for entering the game.")
    )
)]
pub async fn get_game(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
) -> Json<DetailedResponse<GameInfo>> {
    let mut response: DetailedResponse<GameInfo> = DetailedResponse::new(GameInfo::default());
    let mut game_response: DetailedResponse<Game> = DetailedResponse::new(Game::new());

    let mut repository = Repository::new(&mongo, "games");
    game_response
        .run(|a| repository.get_by_document(a, doc! { "generated_pass": pass }))
        .await;
    if let Err(e) = game_to_info(game_response.data.clone(), &mut response.data).await {
        response = response.set_code(Some(e)).clone();
    }

    Json(response.clone())
}

#[utoipa::path(
    post,
    path = "/api/game",
    request_body = GameCreation,
    responses((
        status = 200,
        description = "Found class from database",
        body = GameInfoDetailedResponse 
    ),
    (
        status = 304,
        description = "Could not find class from database",
        body = GameInfoDetailedResponse 
    ),
    (
        status = 500,
        description = " Internal error occured",
        body = GameInfoDetailedResponse 
    ))
)]
pub async fn start_game(
    Extension(mongo): Extension<Database>,
    Json(request): Json<GameCreation>,
) -> Json<DetailedResponse<GameInfo>> {
    let mut game_response = DetailedResponse::new(Game::new());
    let mut response: DetailedResponse<GameInfo> = DetailedResponse::new(GameInfo::default());

    let body: Game = Game::start(request).await;
    let mut repository = Repository::new(&mongo, "games");
    let mut queue_repository = Repository::new(&mongo, "game_queues");

    game_response
        .run(|a| repository.insert_one(body.clone(), a))
        .await;
    if let Err(e) = game_to_info(game_response.data.clone(), &mut response.data).await {
        response = response.set_code(Some(e)).clone();
    }

    // create a queue response and add the generate pass
    // to the queue response 

    let mut queue_response = DetailedResponse::new(Queue::new());

    queue_response.data.game = response.clone().data.game_pass;

    queue_response
        .run(|a| queue_repository.insert_one(
                a.clone().data,
                a
        )).await;

    Json(response)
}

#[utoipa::path(
    post,
    path = "/api/game/{pass}",
    request_body = NewCharacter,
    responses((
        status = 200,
        description = "Added Character to Game",
        body = CharAddedDetailedResponse
    ),
    (
        status = 400,
        description = "Bad request",
        body = CharAddedDetailedResponse
    ),
    (
        status = 404,
        description = "Could not find class from database",
        body = CharAddedDetailedResponse
    ),
    (
        status = 500,
        description = " Internal error occured",
        body = CharAddedDetailedResponse
    )),
    params(
        ("pass" = String, Path, description = "Password for entering the game.")
    )
)]
pub async fn add_character(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>,
    Json(request): Json<NewCharacter>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut char_response: DetailedResponse<Character> = DetailedResponse::new(Character::new());
    let mut class_response: DetailedResponse<Class> = DetailedResponse::new(Class::new());
    let mut game_response: DetailedResponse<Game> = DetailedResponse::new(Game::new());
    let mut ability_response = DetailedResponse::new(Vec::<AbilityModel>::new());

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut class_repo = Repository::<Class>::new(&mongo, "classes");
    let mut ability_repo = Repository::<AbilityModel>::new(&mongo, "abilities");

    ability_response
        .run(|a| {
            ability_repo.get_all(a)
        })
        .await;


    class_response
        .run(|a| class_repo.get_by_document(a, doc! { 
            "class_enum": request.char_class.to_string()
        }))
        .await;

    if let Progress::Failing(f) = class_response.success {
        let mut resp = DetailedResponse::new(char_response.clone().data.as_response());
        return Json(resp.set_code(Some(f)).clone());
    }

    game_response
        .run(|a| game_repo.get_by_document(a, doc! { "generated_pass": pass.clone() }))
        .await;

    let ch = Character::new_game(
        game_response.data.game_id,
        request,
        class_response.data.clone(),
    )
    .await;

    // turn into match
    if let Ok(character) = ch {
        char_response.data = character;
    } else if let Err(e) = ch {
        let mut resp = DetailedResponse::new(char_response.clone().data.as_response());
        return Json(resp.set_code(Some(e)).clone());
    }
    char_response
        .run(|mut a| {
            async {
                ability_response.data.iter().for_each(|b| {
                    a.data.char_deck.push(b.as_response());
                });
                a
            }
        })
        .await;

    // Remove to appease warnings
    let mut char_to_append: mongodb::bson::Document = doc! {};
    match mongodb::bson::ser::to_document(&char_response.data.clone()) {
        Ok(d) => {
            char_to_append = d;
        }
        Err(e) => {
            let mut resp = DetailedResponse::new(char_response.clone().data.as_response());
            return Json(
                resp.set_code(Some(crate::data_types::common::APIError::new(
                    StatusCode::BAD_REQUEST,
                    e.to_string(),
                )))
                .clone(),
            );
        }
    }

    game_response
        .run(|a| {
            game_repo.update_one(
                doc! { "generated_pass": pass.clone()},
                doc! { "$push": {
                    "game_chars": char_to_append
                }},
                a,
            )
        })
        .await;

    let mut resp = DetailedResponse::new(char_response.clone().data.as_response());
    resp.absorb(&mut char_response);
    println!("{:#?}", resp);
    Json(resp)
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

pub mod game_routes {
    pub use super::add_character;
    pub use super::get_game;
    pub use super::get_games;
    pub use super::start_game;
    pub use super::get_game_queue;
    pub use super::take_turn;
}
