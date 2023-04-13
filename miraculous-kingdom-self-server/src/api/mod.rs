pub mod character_api;
pub mod class_api;
pub mod game_api;
pub mod season_api;

pub mod routes {
    use ::axum::{
        routing::{get, post},
        Router,
    };

    pub use super::character_api::character_routes;
    pub use super::class_api::class_routes;
    pub use super::game_api::game_routes;
    pub use super::season_api::season_routes;

    pub fn construct_api_router() -> Router {
        let class_route = Router::new()
            .route("/", get(class_routes::get_all)) // G
            .route("/:id", get(class_routes::get)); // G
        let game_route = Router::new()
            .route(
                "/",
                get(game_routes::get_all) // G
                    .post(game_routes::start_game),
            ) // G
            .route(
                "/:pass",
                get(game_routes::get) // G
                    .post(game_routes::add_character),
            ); // x, x
        let character_route = Router::new()
            .route("/:secret", get(character_routes::get_characters))
            .route(
                "/:secret/:pass",
                get(character_routes::get_character_for_game),
            );
        let season_route = Router::new()
            .route("/", get(season_routes::get_all))
            .route("/:id", get(season_routes::get))
            .route("/roll", get(season_routes::roll));

        Router::new()
            .nest("/class", class_route)
            .nest("/game", game_route)
            .nest("/character", character_route)
            .nest("/season", season_route)
    }
}
