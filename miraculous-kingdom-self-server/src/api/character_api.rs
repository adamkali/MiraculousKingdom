use crate::data_types::{
    characters::{Character, CharacterResponse, Ability},
    common::{
        CharDetialedResponse, DetailedResponse, MKModel, Repository,
        VecCharDetailedResponse, APIError 

    },
    engine::Game,
};
use axum::{extract::Path, Extension, Json, http::StatusCode};
use mongodb::{bson::{self, doc}, Database};
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
        .await;

    char_response = find_char_in_game(char_response, &mut game_response, secret).await;

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
    let mut resp = input;
    (0_u8..4_u8).for_each(|_| 
        resp.data.char_hand.push(resp.data.char_deck.pop().unwrap())
    );
    println!("{:?}", resp.data.char_hand);
    resp
}

async fn draw(
    input: DetailedResponse<Character>
) -> DetailedResponse<Character> {
    let mut resp = input;
    resp.data.char_hand.push(resp.data.char_deck.pop().unwrap());
    resp
}

async fn reload(
    input: DetailedResponse<Character>
) -> DetailedResponse<Character> {
    let mut resp = input.clone();
    let temp = resp.clone().data.char_discard;
    resp.data.char_discard = Vec::<Ability>::new();
    resp.data.char_deck = temp;
    shuffle(resp).await
}

async fn find_char_in_game(
    char_response: DetailedResponse<Character>,
    game_response: &mut DetailedResponse<Game>,
    secret: String,
) -> DetailedResponse<Character> {
    let mut a: DetailedResponse<Character> 
        = char_response;
    match game_response.data
        .game_chars
        .iter()
        .find(|x| 
              x.secret == secret
    ) {
        Some(e) => {
            a.data = e.clone();              
        },
        None => {
            a.set_code(Some(APIError::new( StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Could not find character with the sercret {:?}",
                    secret
                ))));
        }
    }
    a
}


#[utoipa::path(
    get,
    path = "/api/character/init_hand/{secret}/{pass}",
    responses((
        status = 200, 
        description = "Initialized the hand of a player for the game", 
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
            description = "String set by the user to get their data from the game"
        ),
        (
            "pass" = String, 
            Path, 
            description = "String set by the user to get the game"
        ),
    )
)]
pub async fn init_hand(
    Extension(mongo): Extension<Database>,
    Path((secret, pass)): Path<(String, String)>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut game_response = DetailedResponse::new(Game::new());
    let mut chatacterresp = DetailedResponse::new(Character::new());
    let mut char_response = DetailedResponse::new(chatacterresp.clone().data.as_response());

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut char_repo = Repository::<Character>::new(&mongo, "characters");

    game_response
        .run(|a| {
            game_repo.get_by_document(
                a,
                doc! { "generated_pass": pass }
            )
        })
        .await;

    chatacterresp = find_char_in_game(chatacterresp, &mut game_response, secret).await;
    chatacterresp
        .run(|a| {
            shuffle(a)
        })  
        .await;
    chatacterresp = draw_new_hand(chatacterresp).await;

    chatacterresp
        .run(|a| {
            char_repo.update_one(
                doc! { "secret": a.data.clone().secret },
                doc! { "$set": mongodb::bson::to_bson(&a.data).unwrap() },
                a
            )
        })
        .await;

    game_response
        .data
        .clone()
        .game_chars
        .iter()
        .for_each(|x| {
            if x.char_name == chatacterresp.data.clone().char_name {
                let _ = &(chatacterresp.data);
            } 
        });

    game_response
        .absorb(&mut chatacterresp)
        .run(|a| {
            game_repo.update_one(
                doc! { "generated_pass"
                    : a.clone().data.generated_pass },
                doc! { "$set": {
                    "game_chars": bson::to_bson(
                        &a.data.game_chars
                    ).unwrap()
                }},
                a
            )
        })
        .await;

    char_response.absorb(&mut game_response);
    char_response.data = chatacterresp.data.as_response();
    Json(char_response)
}


#[utoipa::path(
    get,
    path = "/api/character/draw/{number}/{secret}/{pass}",
    responses((
        status = 200, 
        description = "Draw an Ability for the player put in", 
        body = CharDetialedResponse,
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = CharDetialedResponse 
    )),
    params(
        (
            "number" = u8, 
            Path, 
            description = "String set by the user to get their data from the game"
        ),
        (
            "secret" = String, 
            Path, 
            description = "String set by the user to get their data from the game"
        ),
        (
            "pass" = String, 
            Path, 
            description = "String set by the user to get the game"
        ),
    )
)]
pub async fn draw_card(
    Extension(mongo): Extension<Database>,
    Path((number, secret, pass)): Path<(u8, String, String)>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut game_response = DetailedResponse::new(Game::new());
    let mut chatacterresp = DetailedResponse::new(Character::new());
    let mut char_response = DetailedResponse::new(
        chatacterresp.clone().data.as_response()
    );

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut char_repo = Repository::<Character>::new(&mongo, "characters");

    game_response
        .absorb(&mut char_response)
        .run(|a| {
            game_repo.get_by_document(
                a,
                doc! { "generated_pass": pass }
                )
        })
        .await;

    chatacterresp = find_char_in_game(chatacterresp, &mut game_response, secret).await;

    for _ in 0..number-1 {
        if chatacterresp.clone().data.char_deck.is_empty() {
           chatacterresp 
                .run(|a| {
                    reload(a) 
                }).await
                .run(|a| {
                    draw(a)
                }).await;
        } else {
           chatacterresp 
                .run(|a| {
                    draw(a)
                }).await;
        }
    }

    game_response.data.game_chars = game_response
        .data
        .clone()
        .game_chars
        .iter()
        .map(|x| {
            if x.char_name == chatacterresp.data.clone().char_name {
                chatacterresp.clone().data
            } else { x.clone() }
        }).collect::<Vec<_>>();


    game_response
        .absorb(&mut chatacterresp)
        .run(|a| {
            game_repo.update_one(
                doc! { "generated_pass"
                    : a.clone().data.generated_pass },
                doc! { "$set": {
                    "game_chars": bson::to_bson(
                        &a.data.game_chars
                    ).unwrap()
                }},
                a
            )
        })
        .await;


    char_response.absorb(&mut game_response);
    char_response.data = chatacterresp.data.as_response();
    Json(char_response)
}

#[utoipa::path(
    get,
    path = "/api/character/discard/{secret}/{pass}",
    responses((
        status = 200, 
        description = "Discard an Ability for the player put in", 
        body = CharDetialedResponse,
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = CharDetialedResponse 
    )),
    request_body = Ability,
    params(
        (
            "secret" = String, 
            Path, 
            description = "String set by the user to get their data"
        ),
        (
            "pass" = String, 
            Path, 
            description = "String set by the user to get their data"
        ),
    )
)]
pub async fn discard_card(
    Extension(mongo): Extension<Database>,
    Path((secret, pass)): Path<(String, String)>,
    Json(ability): Json<Ability>,
) -> Json<DetailedResponse<CharacterResponse>> {
    let mut game_response = DetailedResponse::new(Game::new());
    let mut chatacterresp = DetailedResponse::new(Character::new());
    let mut char_response = DetailedResponse::new(chatacterresp.clone().data.as_response());

    let mut game_repo = Repository::<Game>::new(&mongo, "games");
    let mut char_repo = Repository::<Character>::new(&mongo, "characters");

    game_response.run(|a| {
        game_repo.get_by_document(
            a,
            doc! { "generated_pass": pass },
        )
    })
    .await;

    chatacterresp = find_char_in_game(chatacterresp, &mut game_response, secret).await;

    chatacterresp.absorb(&mut char_response.clone());
    let inex: usize = chatacterresp
                        .clone()
                        .data
                        .char_hand
                        .iter()
                        .position(|x| *x.ability_name == ability.ability_name )
                        .unwrap();
    
    chatacterresp.data.char_hand.remove(inex);

    chatacterresp.data.char_discard.push(ability);
    chatacterresp
        .run(|a| {
            char_repo.update_one(
                doc! { "char_name": a.data.clone().char_name },
                doc! { "$set": {
                    "char_hand": bson::to_bson(
                        &a.data.char_hand
                    ).unwrap(),
                    "char_discard": bson::to_bson(
                        &a.data.char_discard
                    ).unwrap(),

                }},
                a
            )
        })
        .await;

    game_response.data.game_chars = game_response
        .data
        .clone()
        .game_chars
        .iter()
        .map(|x| {
            if x.char_name == chatacterresp.data.clone().char_name {
                chatacterresp.clone().data
            } else { x.clone() }
        }).collect::<Vec<_>>();


    game_response
        .absorb(&mut chatacterresp)
        .run(|a| {
            game_repo.update_one(
                doc! { "generated_pass"
                    : a.clone().data.generated_pass },
                doc! { "$set": {
                    "game_chars": serde_json::to_string(
                        &a.data.game_chars
                    ).unwrap()
                }},
                a
            )
        })
        .await;

    char_response.data = chatacterresp.data.as_response();
    char_response.absorb(&mut game_response);
    Json(char_response)
}

pub mod character_routes {
    pub use super::get_character_for_game;
    pub use super::get_characters;
    pub use super::init_hand;
    pub use super::draw_card;
    pub use super::discard_card;
}
