use axum::{
    response::Redirect,
    extract::State
};

pub async fn rx_index(State(_mk): crate::MKState) -> Redirect { Redirect::permanent("/index-reload") }

pub async fn rx_ability_test(State(_mk): crate::MKState) -> Redirect { Redirect::permanent("/ability-test") }
