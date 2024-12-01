// to be created
//
// /api/ability/pool
// TODO: nice to have /api/ability/{id}
// /api/ability/hand
// /api/ability/hand/draw
// /api/ability/hand/draw/scientist
// /api/ability/deck
// /api/ability/deck/{id}
//

use crate::data_types::{
    characters::{Character, CharacterResponse, Ability},
    common::{
        CharDetialedResponse, DetailedResponse, MKModel, Progress, Repository,
        VecCharDetailedResponse,
    },
    engine::Game,
};
use axum::{extract::Path, Extension, Json};
use mongodb::{bson::doc, Database};


// /api/ability/
// WARNING: This will need to be changed to Ability response or AbilityModel to deal with objectID;
pub async fn get_abilities(
    Extension(mongo): Extension<Database>,
) -> Json<DetailedResponse<Vec<Ability>>> {
    let mut response = DetailedResponse::new(Vec::<Ability>::new());
    let mut repository = Repository::<Ability>::new(&mongo, "abilities");

    response.run(|a| repository.get_all(a)).await;

    Json(response)
}


