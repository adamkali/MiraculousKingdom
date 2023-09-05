use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::extract::State;

use crate::data::models::Ability;
use crate::utils::get_all;

pub async fn get_abilites(State(mk): crate::MKState) -> impl IntoResponse {
    match get_all::<Ability>(&mk.lock().await.config, "ability").await {
        Ok(abilities) => {
            let mut html: String = "<div class='flex flex-col w-full h-full'>".to_string();
            
            abilities
                .iter()
                .for_each(|a| {
                    html.push_str(Into::<String>::into(a.clone()).as_str());
                });
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "text/html")
                ],
                html 
            ).into_response()
        },
        Err(e) => {
            (
                Into::<StatusCode>::into(e.clone()),
                [
                    (header::CONTENT_TYPE, "text/html")
                ],
                format!("<div/>{}<div>", e)
            ).into_response()
        }
    }
}

