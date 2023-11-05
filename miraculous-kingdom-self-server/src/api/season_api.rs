use crate::data_types::{
    common::{
        verify_id, APIError, DetailedResponse, MKModel, Repository, RewardTypes,
        SeasonDetailedResponse, SeasonsDetailedResponse,
    },
    engine::{Season, SeasonResponse},
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use rand::seq::SliceRandom;

#[utoipa::path(
    get,
    path = "/api/season",
    responses((
        status = 200, 
        description = "Listed classes from database", 
        body = SeasonsDetailedResponse 
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = SeasonsDetailedResponse 
    ))
)]
pub async fn get_seasons(
    Extension(mongo): Extension<Database>,
) -> Json<DetailedResponse<Vec<SeasonResponse>>> {
    let mut response: DetailedResponse<Vec<Season>> = DetailedResponse::new(Vec::<Season>::new());
    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    response.run(|a| repository.get_all(a)).await;
    let mut res: DetailedResponse<Vec<SeasonResponse>> = DetailedResponse::new(Vec::new());
    res.absorb(&mut response.clone());
    response
        .data
        .iter()
        .for_each(|a| res.data.push(a.clone().as_response()));

    Json(res)
}

#[utoipa::path(
    get,
    path = "/api/season/{id}",
    responses((
        status = 200, 
        description = "Found class from database", 
        body = SeasonDetailedResponse
    ),
    (
        status = 400, 
        description = "Bad Request: id", 
        body = SeasonDetailedResponse
    ),
    (
        status = 500, 
        description = " Internal error occured", 
        body = SeasonDetailedResponse
    )),
    params(
        ("id" = String, Path, description = "ObjectId for mongodb")
    )
)]
pub async fn get_season(
    Extension(mongo): Extension<Database>,
    Path(id): Path<String>,
) -> impl axum::response::IntoResponse {
    let mut response: DetailedResponse<Season> = DetailedResponse::new(Season {
        event_id: ObjectId::new(),
        event_name: String::new(),
        event_desc: String::new(),
        event_length: 1,
        event_reward: RewardTypes::None,
    });

    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    if let Err(e) = verify_id(id, &mut response.data.event_id).await {
        response.clone().set_code(Some(e));
    }

    response
        .run(|a| repository.get_by_oid(a.clone(), a.data.event_id))
        .await;

    let mut res: DetailedResponse<SeasonResponse> =
        DetailedResponse::new(response.data.as_response());
    res.success = response.success;
    
    
}

#[utoipa::path(
    get,
    path = "/api/season/roll",
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
    let mut response: DetailedResponse<SeasonResponse> = DetailedResponse::new(Season {
        event_id: ObjectId::new(),
        event_name: String::new(),
        event_desc: String::new(),
        event_length: 1,
        event_reward: RewardTypes::None,
    }.as_response());

    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    seasons_response
        .run(|a| repository.get_all(a))
        .await;

    println!("{}", serde_json::to_string_pretty(&seasons_response).unwrap());

    match seasons_response.data.choose(&mut rand::thread_rng()) {
        Some(a) => {
            response.data = a.clone().as_response();
        }
        None => {
            response.set_code(Some(APIError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Counld not get a random event".to_string(),
            )));
        }
    }

    response.success = seasons_response.success;
    Json(response)
}

pub mod season_routes {
    pub use super::get_season;
    pub use super::get_seasons;
    pub use super::roll;
}
