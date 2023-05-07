use crate::data_types::{
    characters::{Character, CharacterResponse, Ability},
    common::{
        CharDetialedResponse, DetailedResponse, MKModel, Progress, Repository,
        VecCharDetailedResponse, APIError,

    },
    engine::Game,
};
use axum::{extract::Path, Extension, Json, http::StatusCode};
use mongodb::{bson::doc, Database};
use rand::seq::SliceRandom;

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
    response
        .data
        .iter()
        .for_each(|a| resp.data.push(a.clone().as_response()));
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
    Path((secret, pass)): Path<(String, String)>,
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

async fn shuffle(
    input: DetailedResponse<Character>
) -> DetailedResponse<Character> {
    let mut resp = input.clone();
    let mut temp0: Vec<Ability> = input.data.char_deck;
    let mut temp1: Vec<Ability> = Vec::<Ability>::with_capacity(temp0.len());
    temp0.clone().iter_mut().for_each(|_| {
        let temp = temp0.iter()
                        .as_slice()
                        .choose(&mut rand::thread_rng())
                        .unwrap()
                        .clone();

        let index = temp0.iter()
                         .position(|x| x.clone().ability_name == temp.clone().ability_name);

        match index {
            Some(i) => {
                temp0.remove(i);
                temp1.push(temp);
            }
            None => {
                resp.set_code(Some(APIError {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: "Could not shuffle".to_string()
                }));
            }
        }
    });
    resp.data.char_deck = temp1;
    
    resp
}


async fn draw_new_hand(
    input: DetailedResponse<Character>
) -> DetailedResponse<Character> {
    let mut resp = input.clone();
    (0_u8..4_u8).for_each(|_| resp.data.char_hand.push(resp.data.char_deck.pop().unwrap().clone()));
    resp
}

async fn draw(
    input: DetailedResponse<Character>
) -> DetailedResponse<Character> {
    let mut resp = input.clone();
    resp.data.char_hand.push(resp.data.char_deck.pop().unwrap().clone());
    resp
}

// /api/char/init_hand
pub async fn init_hand(
    Extension(mongo): Extension<Database>,
    Json((secret, pass)): Json<(String, String)>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut char_response = DetailedResponse::new(Character::new());

    char_response
        .run(|a| {
            shuffle(a)
        })  
        .await
        .run(|a| {
            draw_new_hand(a)
        })
        .await;

    let resp = DetailedResponse::new(char_response.data.as_response());
    Json(resp)
}

// /api/char/draw
pub async fn draw_card(
    Extension(mongo): Extension<Database>,
    Json((secret, pass)): Json<(String, String)>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut char_response = DetailedResponse::new(Character::new());

    char_response
        .run(|a| {
            shuffle(a)
        })  
        .await
        .run(|a| {
            draw_new_hand(a)
        })
        .await;

    let resp = DetailedResponse::new(char_response.data.as_response());
    Json(resp)
}

pub mod character_routes {
    pub use super::get_character_for_game;
    pub use super::get_characters;
}




