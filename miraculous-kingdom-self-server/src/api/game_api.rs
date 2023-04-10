use axum::{
    Extension,
    http::StatusCode,
    Json,
    extract::Path,
};
use mongodb::{
    Database,
    bson::{
        doc,
        to_document
    },
};
use utoipa::openapi::{
    RefOr,
    Schema
};
use crate::data_types::{
    engine::*,
    characters::{
        Character,
        NewCharacter,
        Class
    },
    common::DetailedResponse,
    common::Repository,
    common::APIError,
    game_to_info,
    games_to_info,
};
use std::sync::Mutex;

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
pub async fn get_all(
    Extension(mongo): Extension<Database>,
) -> Json<DetailedResponse<Vec<GameInfo>>> {
    let mut response: DetailedResponse<Vec<GameInfo>> =
        DetailedResponse::new(Vec::<GameInfo>::new());

    let mut repository = Repository::new(&mongo, "games");
    let games = Box::new(Vec::<Game>::new());
    response
        .run(|_| repository.get_all(
            *games.clone()
        ))
        .await;
    games_to_info(*games.clone(), &mut response.data).await;

    return Json(response);
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
pub async fn get(
    Extension(mongo): Extension<Database>,
    Path(pass): Path<String>
) -> Json<DetailedResponse<GameInfo>> {
    let mut response: DetailedResponse<GameInfo> = 
        DetailedResponse::new(GameInfo { 
            game_name: "".to_string(), 
            game_ruler: "".to_string(),
            game_chars: Vec::<String>::new(),
        });

    let game = Box::new(Game::new());
    let mut repository = Repository::new(&mongo, "games");
    response = response.run(|a| 
        repository.get_by_document(
            *game.clone(),
            doc! { "generated_pass": pass },
        )).await;
    if let Err(e) = game_to_info(
        *game.clone(), 
        &mut response.data
    ).await {
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
        body = PassDetailedResponse
    ),
    (
        status = 304, 
        description = "Could not find class from database", 
        body = PassDetailedResponse
    ),
    (
        status = 500, 
        description = " Internal error occured", 
        body = PassDetailedResponse
    ))
)]
pub async fn start_game(
    Extension(mongo): Extension<Database>,
    Json(request): Json<GameCreation>
) -> Json<DetailedResponse<String>> {
    let mut response: DetailedResponse<String> =
        DetailedResponse::new(String::new());

    let body: Game = Game::start(request).await;
    let mut repository = Repository::new(&mongo, "games");
    
    response.run(|_| repository.insert_one(body.clone())).await;
    response.data = body.generated_pass;

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
) -> Json<DetailedResponse<Character>> {
    let mut response: DetailedResponse<Character> 
        = DetailedResponse::new(Character::new());
    let class: Class = Class::new();

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut class_repo = Repository::<Class>::new(&mongo, "classes");
    let mut char_repo = Repository::<Character>::new(&mongo, "characters");

    response.run(|_| class_repo.get_by_document(
                class.clone(),
                doc! { "class_name": request.char_class.clone() },
            )).await;
    
    let game = Box::new(Game::new());
    response.clone().run(|_| game_repo.get_by_document(
            *game.clone(),
            doc! { "generated_pass": pass.clone() },
            )).await;

    let ch = Character::new_game(
                        (*game.clone()).game_id,
                        request,
                        class.clone()
                        ).await;
    // turn into match
    if let Ok(character) = ch {
        response.data = character;         
    } else if let Err(e) = ch {
        return Json(response.set_code(Some(e)).clone());
    }
    
    response.clone().run(|a| char_repo.insert_one(a)).await;

    // Remove to appease warnings
    let mut char_to_append: mongodb::bson::Document
        = doc!{};
    // change this to a for loop
    match mongodb::bson::ser::to_document(&response.data.clone()) {
        Ok(d) => {char_to_append = d;},
        Err(e) => {
            return Json(
                response
                    .set_code(Some(
                        crate::data_types::common::APIError::new(
                            StatusCode::BAD_REQUEST,
                            e.to_string(),
                        )
                    ))
                    .clone());
        }
    }

    response.clone().run(|_| game_repo.update_one(
        doc! { "generated_pass": pass.clone()},
        doc! { "$push": { 
            "game_chars": char_to_append 
        }},
    )).await;

    Json(response.clone())
}

pub mod game_routes {
    pub use super::start_game;
    pub use super::add_character;
    pub use super::get;
    pub use super::get_all;
}
