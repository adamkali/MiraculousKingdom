use utoipa::OpenApi;

pub mod auth;
pub mod classes;
use crate::models::detailed_response::*;
use classes::ClassResponse;

#[derive(OpenApi)]
#[openapi(
    info(description = "Miraculous Kingdom public api"),
    servers(
        (url =  "http://127.0.0.1:5150")
    ),
    paths(
        classes::get_all
    ),
    components(
        schemas(
            ClassListDetailedResponse,
            ClassResponse
        ),
    ),
)]
pub struct OpenAPI;
