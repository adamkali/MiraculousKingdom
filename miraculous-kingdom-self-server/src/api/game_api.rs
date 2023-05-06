use crate::data_types::{
    characters::{Character, CharacterResponse, Class, NewCharacter},
    common::DetailedResponse,
    common::MKModel,
    common::Progress,
    common::Repository,
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
    let mut game_response: DetailedResponse<Vec<Game>> = DetailedResponse::new(Vec::<Game>::new());

    let mut repository = Repository::new(&mongo, "games");
    game_response.run(|a| repository.get_all(a)).await;
    games_to_info(game_response.data, &mut response.data).await;

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
    Json(request): Json<GameCreation>,
) -> Json<DetailedResponse<String>> {
    let mut game_response = DetailedResponse::new(Game::new());

    let body: Game = Game::start(request).await;
    let mut repository = Repository::new(&mongo, "games");

    game_response
        .run(|a| repository.insert_one(body.clone(), a))
        .await;
    let response = DetailedResponse::new(game_response.data.generated_pass);

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

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut class_repo = Repository::<Class>::new(&mongo, "classes");
    let mut char_repo = Repository::<Character>::new(&mongo, "characters");

    println!("{:?}", request.char_class.clone());

    let doc = doc! { "class_enum": request.char_class.to_string() };
    println!("{:?}", doc);

    class_response
        .run(|a| class_repo.get_by_document(a, doc))
        .await;

    if let Progress::Failing(f) = class_response.success {
        let mut resp = DetailedResponse::new(char_response.clone().data.as_response());
        return Json(resp.set_code(Some(f)).clone());
    }

    game_response
        .run(|a| game_repo.get_by_document(a, doc! { "generated_pass": pass.clone() }))
        .await;

    println!("{:?}", class_response.data.clone());

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
        .clone()
        .run(|a| char_repo.insert_one(a.clone().data, a))
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
    Json(resp)
}

pub mod game_routes {
    pub use super::add_character;
    pub use super::get_game;
    pub use super::get_games;
    pub use super::start_game;
}
