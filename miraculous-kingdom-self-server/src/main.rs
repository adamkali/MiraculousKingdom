pub mod api;
pub mod data_types;

use axum::{
    routing::*,
    http::method::Method,
    Extension 
};
use tower_http::cors::{CorsLayer, Any};
//use std::sync::Arc;
//use data_types::engine::Game;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use data_types::common::*;
use data_types::characters::*;
use data_types::clock::*;
use data_types::engine::*;
use data_types::might::*;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)] 
    #[openapi(
        paths(
            api::class_api::get_all,
            api::class_api::get,
            api::game_api::get_all,
            api::game_api::start_game,
            api::game_api::get,
            api::game_api::add_character,
            api::character_api::get_character_for_game,
            api::character_api::get_characters,
            api::season_api::get_all,
            api::season_api::get,
            api::season_api::roll
        ),
        components(
            schemas(
                Character, NewCharacter, Class,
                VecClassDetailedResponse, ClassDetailedResponse,
                GameInfoDetailedResponse, GamesInfoDetailedResponse,
                PassDetailedResponse, CharAddedDetailedResponse,
                VecCharDetailedResponse, CharDetialedResponse,
                Ability, Might, MightStat, Clock, GameInfo,
                GameCreation, MightRequirement, Progress, MightEnum,
                Season, RewardTypes, RollTier
            ),
        ),
        tags(
            (name = "Miraculous Kingdom", description = "Miraculous Kingdom SwaggerUi")
        )
    )]
    struct APIDoc;

    let uri = "mongodb://root:mk2023!@localhost:8100";
    let client_opt = mongodb::options::ClientOptions::parse(uri).await.unwrap();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let mongo_client = mongodb::Client::with_options(client_opt).unwrap().database("mkdb");
        
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", APIDoc::openapi()))
        .route("/", get(|| async { 
            "And the serve did not go down, quoth the admin \"Nevermore\"" 
            }))
        .nest("/api", api::routes::construct_api_router())
        .layer(cors)
        .layer(Extension(mongo_client));
        

    axum::Server::bind(&"0.0.0.0:8050".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
