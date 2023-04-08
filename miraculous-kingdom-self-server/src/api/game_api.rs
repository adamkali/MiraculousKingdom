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
};
use futures::stream::TryStreamExt;

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

    let collection = mongo.clone()
                          .collection::<Game>("games");
    let cursor = collection.find_one(
        doc! { "generated_pass": pass.clone() }, None 
        ).await;
    match cursor {
        Ok(c) => {
            if let Some(result) = c {
                let chars: Vec<String> = Vec::new();
                response.data = GameInfo{
                    game_ruler: result.game_ruler,
                    game_name: result.game_name,
                    game_chars: result.game_chars
                                .iter()
                                .clone()
                                .map(|a| a.char_name.clone())
                                .collect(),
                };
                response.set_code(StatusCode::OK, String::new());
            } else {
                response.set_code(StatusCode::NOT_FOUND, 
                          format!("could not find game with password {}", pass));
            }
        },
        Err(e) => {
            response.message = e.to_string();
            response.set_code(StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string());
        }
    }  
    Json(response)
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
        status = 404, 
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

    let collection = mongo.clone()
                          .collection::<Game>("games");
    let cursor = collection.insert_one(body.clone(), None).await;

    match cursor {
        Ok(_) => {
            response.set_code(StatusCode::OK, "Created Successfully".to_string());
            response.data = body.generated_pass;
        },
        Err(e) => {
            response.set_code(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    }
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
    let mut class: Class = Class::new();

    let game_coll = mongo
                   .clone()
                   .collection::<Game>("games");
    let class_coll = mongo
                    .clone()
                    .collection::<Class>("classes");
    let char_coll = mongo
                    .clone()
                    .collection::<Character>("characters");

    let class_c = class_coll.find_one(
                    doc! { "class_name": request.char_class.clone() },
                    None 
                    ).await;

    match class_c {
        Ok(c) => {
            if let Some(result) = c {
                class = result;
            }
        },
        Err(e) => {
            response.set_code(StatusCode::BAD_REQUEST,e.to_string());
            return Json(response);
        }
    }
    
    let game: Game;
    let game_c = game_coll.find_one(
        doc! { "generated_pass": pass.clone() }, None 
        ).await;
    match game_c {
        Ok(c) => {
            if let Some(result) = c {
                game = result;
            } else {
                response.set_code(StatusCode::NOT_FOUND, 
                    format!("There is no game with code: {}", pass));
                return Json(response);
            }
        },
        Err(e) => {
            
            response.set_code(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
            return Json(response);
        }
    }
    let ch = Character::new_game(
                        game.game_id,
                        request,
                        class
                        ).await;
    // turn into match
    if let Ok(character) = ch {
        response.data = character;         
    } else if let Err(e) = ch {
        return Json(response.set_code(e.status_code, e.message).clone());
        
    }
    
    let char_c = char_coll.insert_one(response.data.clone(), None).await;
    if let Err(e) = char_c {
        return Json(
            response
                .set_code(StatusCode::NOT_MODIFIED, e.to_string())
                .clone()
            );
    }
    // Remove to appease warnings
    let mut char_to_append: mongodb::bson::Document
        = doc!{};
    // change this to a for loop
    match mongodb::bson::ser::to_document(&response.data.clone()) {
        Ok(d) => {char_to_append = d;},
        Err(e) => {
            return Json(
                response
                    .set_code(StatusCode::NOT_MODIFIED, e.to_string())
                    .clone()
                );
        }
    }

    if !response.success { return Json(response); }
    // TODO(adam): remove the character if no good!!!!!
    // FIXME(adam): 🙏
    let update_game = game_coll.update_one( 
        doc! { "generated_pass": pass },
        doc! { "$push": { 
            "game_chars": char_to_append
        }},
        None
    ).await;

    match update_game {
        Ok(c) => {
            if c.modified_count == 0 {
                response.set_code(StatusCode::NOT_MODIFIED, 
                                  "could not update the game".to_string());
            }
        },
        Err(e) => {
            return Json(response
                        .set_code(
                            StatusCode::INTERNAL_SERVER_ERROR, 
                            e.to_string()
                        ).clone());
        }
    }
    response.set_code(StatusCode::OK, String::new());
    Json(response)
}

pub mod game_routes {
    pub use super::start_game;
    pub use super::add_character;
    pub use super::get;
    pub use super::get_all;
}
