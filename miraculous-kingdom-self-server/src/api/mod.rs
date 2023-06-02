pub mod character_api;
pub mod class_api;
pub mod game_api;
pub mod season_api;
pub mod ability_api;
pub mod queue_api;


pub mod routes {
    use ::axum::{
        routing::{get, post, put},
        Router,
    };

    pub use super::character_api::character_routes;
    pub use super::class_api::class_routes;
    pub use super::game_api::game_routes;
    pub use super::season_api::season_routes;
    pub use super::queue_api::queue_routes;

    pub fn construct_api_router() -> Router {
        let class_route = Router::new()
            .route("/", get(class_routes::get_classes)) // G
            .route("/:id", get(class_routes::get_class)); // G
        let game_route = Router::new()
            .route(
                "/",
                get(game_routes::get_games) // G
                    .post(game_routes::start_game),
            ) // G
            .route(
                "/:pass",
                get(game_routes::get_game) // G
                    .post(game_routes::add_character),
            ); // x, x
        let character_route = Router::new()
            .route("/:secret", get(character_routes::get_characters))
            .route(
                "/:secret/:pass",
                get(character_routes::get_character_for_game),
            )
            .route(
                "/init_hand/:secret/:pass",
                put(character_routes::init_hand),
            )
            .route(
                "/draw/:number/:secret/:pass",
                put(character_routes::draw_card),
            )
            .route(
                "/discard/:secret/:pass",
                put(character_routes::discard_card),
            );
        let season_route = Router::new()
            .route("/", get(season_routes::get_seasons))
            .route("/:id", get(season_routes::get_season))
            .route("/roll", get(season_routes::roll));

        let queue_route = Router::new()
            .route("/:pass", get(queue_routes::get_queue))
            .route("/turn/:pass", post(queue_routes::take_turn))
            .route("/season/:pass", post(queue_routes::set_season));


        Router::new()
            .nest("/class", class_route)
            .nest("/game", game_route)
            .nest("/character", character_route)
            .nest("/season", season_route)
            .nest("/queue", queue_route)
            
    }
}
