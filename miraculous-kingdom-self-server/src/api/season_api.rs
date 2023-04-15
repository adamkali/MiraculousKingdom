use crate::data_types::common::{
    verify_id, APIError, DetailedResponse, Repository, RewardTypes, Season, SeasonDetailedResponse,
    SeasonsDetailedResponse,
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
pub async fn get_seasons(Extension(mongo): Extension<Database>) -> Json<DetailedResponse<Vec<Season>>> {
    let mut response: DetailedResponse<Vec<Season>> = DetailedResponse::new(Vec::<Season>::new());
    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    response.run(|a| repository.get_all(a)).await;
    Json(response.clone())
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
) -> Json<DetailedResponse<Season>> {
    let mut response: DetailedResponse<Season> = DetailedResponse::new(Season {
        event_id: ObjectId::new(),
        event_name: String::new(),
        event_desc: String::new(),
        event_length: 1,
        event_reward: RewardTypes::Experience(1),
    });

    let mut repository = Repository::<Season>::new(&mongo, "seasons");

    if let Err(e) = verify_id(id, &mut response.data.event_id).await {
        response.clone().set_code(Some(e));
    }

    Json(
        response
            .run(|a| repository.get_by_oid(a.clone(), a.data.event_id))
            .await,
    )
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
pub async fn roll(Extension(mongo): Extension<Database>) -> Json<DetailedResponse<Season>> {
    let mut seasons_response: DetailedResponse<Vec<Season>> =
        DetailedResponse::new(Vec::<Season>::new());
    let mut response: DetailedResponse<Season> = DetailedResponse::new(Season {
        event_id: ObjectId::new(),
        event_name: String::new(),
        event_desc: String::new(),
        event_length: 1,
        event_reward: RewardTypes::Experience(1),
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

    Json(response.clone())
}

pub mod season_routes {
    pub use super::get_seasons;
    pub use super::get_season;
    pub use super::roll;
}
