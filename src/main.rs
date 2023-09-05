mod api;
mod utils;
mod data;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get, Router,
};
use tokio::sync::Mutex;
use tracing::{info, error};
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::path::Path as StdPath;
use surrealdb::{
    Surreal,
    engine::remote::ws::{
        Ws,
        Client
    },
    opt::auth::Root
};

pub use api::*;

#[derive(Clone)]
pub struct MiraculousKingdomDatabaseConfig {
    pub ns: String,
    pub db: String,
    pub username: String,
    pub password: String,
}

pub struct MiraculousKingdomState {
    pub db: Surreal<Client>,
    pub config: MiraculousKingdomDatabaseConfig,
}

pub type MKState = State<Arc<Mutex<MiraculousKingdomState>>>;

#[tokio::main]
async fn main() {

    dotenv::dotenv().expect("Could not load the .env file");
    // Get the environment variables from the .env
    let db_user = std::env::var("DB_USER").expect("DB_USER not set");
    let db_pass = std::env::var("DB_PASS").expect("DB_PASS not set");
    let db_ns = std::env::var("DB_NS").expect("DB_NS not set");
    let db_db = std::env::var("DB_DB").expect("DB_DB not set");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "miraculous_kingdom=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mk_state: Arc<Mutex<MiraculousKingdomState>>;

    match Surreal::new::<Ws>("localhost:3100").await {
        Ok(db) => {
            tracing::debug!("Connected to database");

            if let Err(e) = db.signin(Root {
                username: &db_user,
                password: &db_pass,
            })
            .await
            {
                tracing::error!("Error signing in");
                panic!("Error: {}", e);
            }

            if let Err(e) = db.use_ns(&db_ns).use_db(&db_db).await {
                error!("Error using namespace");
                panic!("Error: {}", e);
            }

            // Create a MiraculousKingdomDatabaseConfig with owned String values
            // that live long enough
            let config = MiraculousKingdomDatabaseConfig {
                ns: db_ns,
                db: db_db,
                username: db_user,
                password: db_pass,
            };

            let migration = utils::migrate(&config).await;

            match migration {
                Ok(_) => info!("Migration Complete"),
                Err(e) => panic!("Error: {}", e),
            }

            mk_state = Arc::new(Mutex::new(MiraculousKingdomState { db, config }));
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    // Compose the routes
    let app = Router::new()
        .route("/", get(index))
        .route("/ability-test", get(ability_test))
        .route("/index-reload", get(index_reload))
        .nest("/rx", api::construct_redirect_api())
        .nest("/api", api::construct_api())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(mk_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


