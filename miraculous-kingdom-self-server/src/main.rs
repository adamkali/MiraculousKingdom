pub mod api;
pub mod data_types;
pub mod ws;

use axum::{http::method::Method, routing::*, Extension};
use std::{net::SocketAddr, path::PathBuf};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::cors::{Any, CorsLayer};
//use std::sync::Arc;
//use data_types::engine::Game;
use data_types::characters::*;
use data_types::clock::*;
use data_types::common::*;
use data_types::engine::*;
use data_types::might::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::class_api::get_classes,
            api::class_api::get_class,
            api::game_api::get_games,
            api::game_api::start_game,
            api::game_api::get_game,
            api::game_api::add_character,
            api::character_api::get_character_for_game,
            api::character_api::get_characters,
            api::season_api::get_seasons,
            api::season_api::get_season,
            api::season_api::roll
        ),
        components(
            schemas(
                CharacterResponse, NewCharacter, ClassResponse,
                VecClassDetailedResponse, ClassDetailedResponse,
                GameInfoDetailedResponse, GamesInfoDetailedResponse,
                PassDetailedResponse, CharAddedDetailedResponse,
                VecCharDetailedResponse, CharDetialedResponse,
                Ability, Might, MightStat, Clock, GameInfo,
                GameCreation, MightRequirement, Progress, MightEnum,
                SeasonResponse, RewardTypes, RollTier, ClassEnum, CharacterState,
                APIError, 
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

    let mongo_client = mongodb::Client::with_options(client_opt)
        .unwrap()
        .database("mkdb");


    // construct tcp and https://
    // using use axum_server::tls_rustls::RustlsConfig
    let ports = Ports {
        http: 8500,
        https: 8050,
    };

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", APIDoc::openapi()))
        .route(
            "/",
            get(|| async { "And the serve did not go down, quoth the admin \"Nevermore\"" }),
        )
        .nest("/api", api::routes::construct_api_router())
        .layer(cors)
        .layer(Extension(mongo_client));

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));

    match RustlsConfig::from_pem_file(
        // XXX: for local
        // PathBuf::from("./certs/cacert.pem").as_path(),
        // PathBuf::from("./certs/privkey.pem").as_path()
        // XXX: for docker!
        PathBuf::from("/working/certs/cacert.pem").as_path(),
        PathBuf::from("/working/certs/privkey.pem").as_path()
    ).await {
        Ok(conf) => { 
            println!("Connection opened on https://{:?}", addr.to_string());
            axum_server::bind_rustls(addr, conf)
                .serve(app.clone().into_make_service())
                .await
                .unwrap();
        },
        Err(e) => {
            println!("{:?}\n cacert: {:#?}\n privkey: {:#?}\n",
                e,
                std::fs::File::open(
                    PathBuf::from("/working/certs/cacert.pem").as_path()
                ).is_ok(),
                std::fs::File::open(
                    PathBuf::from("/working/certs/privkey.pem").as_path()
                ).is_ok()
            );
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    };
}
