use crate::data_types::{
    characters::{
        CharacterResponse,
        Character
    },
    common::{
        CharDetialedResponse, 
        DetailedResponse, 
        Progress, 
        Repository, 
        VecCharDetailedResponse,
        MKModel,
    },
    engine::Game,
};
use axum::{extract::Path, Extension, Json};
use mongodb::{
    bson::doc,
    Database,
};

/// Endpoint to find all characters that the players is participating in for their specific secret.
///
/// # Example
///
/// ```
/// GET /api/character/{secret}
/// ```
///
/// # Parameters
///
/// - `secret`: String set by the user. Should keep the same. For now too lazy to fix 👿
///
/// # Responses
///
/// - `200 OK`: Found characters with secret_code: {secret}
///
///     ```json
///     {
///         "code": 200,
///         "message": "Found characters with secret_code: {secret}",
///         "success": true,
///         "data": [
///             {
///                 "char_name": "character_name",
///                 ...
///             },
///             ...
///         ]
///     }
///     ```
///
/// - `500 Internal Server Error`: An internal error occurred.
///
///     ```json
///     {
///         "code": 500,
///         "success": false
///         "message": "Internal error occurred",
///         "data": []
///     }
///     ```
///
#[utoipa::path(
    get,
    path = "/api/character/{secret}",
    responses((
        status = 200, 
        description = "Found characters with secret_code: {secret}", 
        body = VecCharDetailedResponse,
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = VecCharDetailedResponse 
    )),
    params(
        (
            "secret" = String, 
            Path, 
            description = "String set by the user to get their data"
        )
    )
)]
pub async fn get_characters(
    Extension(mongo): Extension<Database>,
    Path(secret): Path<String>,
) -> Json<DetailedResponse<Vec<CharacterResponse>>> {
    let mut response = DetailedResponse::new(Vec::<Character>::new());

    let mut repository = Repository::<Character>::new(&mongo, "characters");
    response
        .run(|a| {
            repository.get_all_with_filter(
                a,
                doc! {
                    "secret": secret.clone(),
                },
            )
        })
        .await;
    let mut resp = DetailedResponse::new(Vec::<CharacterResponse>::new());
    response.data.iter().for_each(|a| resp.data.push(a.clone().as_response()));
    Json(resp)
}

#[utoipa::path(
    get,
    path = "/api/character/{secret}/{pass}",
    responses((
        status = 200, 
        description = "Found characters with : {secret} and {pass}", 
        body = CharDetialedResponse,
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = CharDetialedResponse 
    )),
    params(
        (
            "secret" = String, 
            Path, 
            description = "String set by the user to get their data"
        ),
        (
            "pass" = String,
            Path,
            description = "String generated by the api for the specific game."
        )
    )
)]
pub async fn get_character_for_game(
    Extension(mongo): Extension<Database>,
    Path(secret): Path<String>,
    Path(pass): Path<String>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut char_response = DetailedResponse::new(Character::new());
    let mut game_response = DetailedResponse::new(Game::new());

    let mut game_repo = Repository::<Game>::new(&mongo, "games");

    game_response
        .run(|a| game_repo.get_by_document(a, doc! { "generated_pass": pass.clone() }))
        .await
        .absorb(&mut char_response);

    if let Progress::Succeeding = char_response.success {
        let _throw: Vec<&mut Character> = game_response
            .data
            .game_chars
            .clone()
            .iter_mut()
            .map(|a| {
                if a.clone().secret == secret {
                    char_response.data = a.clone();
                    a
                } else {
                    a
                }
            })
            .collect();
    }

    let resp = DetailedResponse::new(char_response.data.as_response());
    Json(resp)
}

pub mod character_routes {
    pub use super::get_character_for_game;
    pub use super::get_characters;
}
