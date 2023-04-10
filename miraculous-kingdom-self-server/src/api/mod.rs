pub mod class_api;
pub mod game_api;
pub mod character_api;

pub mod routes {
    use::axum::{
        Router,
        routing::{
            get,
            post
        }
    };
    
    pub use super::class_api::class_routes;
    pub use super::game_api::game_routes;

    pub fn construct_api_router() -> Router {
        let class_route = Router::new()
            .route("/", get(class_routes::get_all)) // G
            .route("/:id", get(class_routes::get)); // G
        let game_route = Router::new()
            .route("/", get(game_routes::get_all) // G 
                       .post(game_routes::start_game)) // G
            .route("/:pass", 
                    get(game_routes::get) // G
                   .post(game_routes::add_character)); // x, x

        Router::new()
            .nest("/class", class_route)
            .nest("/game", game_route)
    }
}
