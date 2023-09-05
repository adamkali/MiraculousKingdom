mod redirect;
mod abilities;
mod classes;

use std::{sync::Arc, path::Path};

use axum::{
    Router,
    routing::get, response::IntoResponse, http::StatusCode,
};
use tokio::sync::Mutex;

pub fn construct_redirect_api() -> Router<Arc<Mutex<crate::MiraculousKingdomState>>> {
    Router::new()
        .route("/index", get(redirect::rx_index))
        .route("/ability-test", get(redirect::rx_ability_test))
                        
}

pub fn construct_api() -> Router<Arc<Mutex<crate::MiraculousKingdomState>>> {
    Router::new()
        .route("/abilities", get(abilities::get_abilites))
}

pub async fn index() -> impl IntoResponse {
    use std::fs::read_to_string;
    use tracing::debug;
    use axum::http::header::*;

    let status: StatusCode;
    let body: String;
    let path = Path::new("static/index.html");

    let res =  read_to_string(path);

    match res {
        Ok(html) => {
            status = StatusCode::OK;
            body = html.clone();
        }
        Err(e) => {
            debug!("Error: {}", e);
            status = StatusCode::NOT_FOUND;
            body = e.to_string();
        }
    }

    (
        status,
        [
            (CONTENT_TYPE, "text/html")
        ],
        body.clone(),
    ).into_response()
}

pub async fn ability_test() -> impl IntoResponse {
    use std::fs::read_to_string;
    use tracing::debug;
    use axum::http::header::*;

    let status: StatusCode;
    let body: String;
    let path = Path::new("./static/ability_test.html");

    let res =  read_to_string(path);

    match res {
        Ok(html) => {
            status = StatusCode::OK;
            debug!("{}", html);
            body = html.clone();
        }
        Err(e) => {
            debug!("Error: {}", e);
            status = StatusCode::NOT_FOUND;
            body = e.to_string();
        }
    }

    (
        status,
        [
            (CONTENT_TYPE, "text/html")
        ],
        body.clone(),
    ).into_response()
}


pub async fn index_reload() -> impl IntoResponse {
    use std::fs::read_to_string;
    use tracing::debug;
    use axum::http::header::*;

    let status: StatusCode;
    let body: String;
    let path = Path::new("static/index_reload.html");

    let res =  read_to_string(path);

    match res {
        Ok(html) => {
            status = StatusCode::OK;
            body = html.clone();
        }
        Err(e) => {
            debug!("Error: {}", e);
            status = StatusCode::NOT_FOUND;
            body = e.to_string();
        }
    }

    (
        status,
        [
            (CONTENT_TYPE, "text/html")
        ],
        body.clone(),
    ).into_response()
}
