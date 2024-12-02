#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::debug_handler;
use serde::Serialize;
use utoipa::ToSchema;
use crate::models::_entities::classes::{self, Model as Class};
use crate::models::detailed_response::*;


#[derive(Serialize, ToSchema)]
pub struct ClassResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub display_name: String,
}

#[utoipa::path(
    get,
    path = "/api/classes",
    responses(
        (status=200, description="get available classes", body = ClassListDetailedResponse)
    ),
)]
#[debug_handler]
pub async fn get_all(State(ctx): State<AppContext>) -> Result<Response> {
    let classes_res = classes::Model::get_all(&ctx.db).await;

    let classes: Vec<Class> = match classes_res {
        Ok(classes) => classes,
        Err(err) => { return DetailedResponse::<Vec<Class>>::fail(None, 404, None, err).json(); }
    };

    DetailedResponse::ok(Some(classes), None).json()
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/classes")
        .add("/", get(get_all))
}
