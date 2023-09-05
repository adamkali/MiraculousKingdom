use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::extract::State;

use crate::data::models::{Ability, ClassRawModel, Class};
use crate::utils::{get_all, FromRawModel};

pub async fn get_classes(State(mk): crate::MKState) -> impl IntoResponse {
    match get_all::<ClassRawModel>(&mk.lock().await.config, "ability").await {
        Ok(classes) => {
            let mut html: String = "<div class='flex flex-col'>".to_string();
             
            let config_clone = &mk.lock().await.config;
            let mut classes_transformed: Vec<Class> = Vec::new();
            for i in classes {
                let class = match i.resolve(config_clone).await {
                    Ok(c) => c,
                    Err(e) => return (
                        Into::<StatusCode>::into(e.clone()),
                        [
                            (header::CONTENT_TYPE, "text/html")
                        ],
                        "<div/>An Error occured in translation in the database.<div>"
                    ).into_response()
                };
                classes_transformed.push(class)
            }

            classes_transformed
                .iter()
                .for_each(|a| {
                    html.push_str(
                        Into::<String>::into(a.clone()).as_str()
                    )
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
