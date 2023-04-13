//use std::sync::Arc;
//use axum::{
//    Router,
//    routing::{
//        get,
//        post,
//    },
//    extract::{
//        Extension,
//        ws::{
//            WebSocketUpgrade,
//            Message,
//            WebSocket,
//        },
//    }
//};
//
//
//use crate::data_types::{
//    engine::{Game, GameInfo, State, Season},
//    characters::Character,
//};

//// a struct for holding the character, and ip of the player.
//pub struct Player {
//    pub character: Character,
//    pub ip: String,
//}
//
//// a struct for holding Game itself as an arc.
//// and the list of players using the Player struct,
//// the current season and the current state of the game.
//pub struct GameListener {
//    pub game: Game,
//    pub players: Vec<Player>,
//    pub current_season: Season,
//    pub current_state: State,
//}
//
//impl GameListener {
//    pub fn new() -> Self {
//        Self {
//            game: Game::new(),
//            players: Vec::new(),
//            current_season: Season::new(),
//            current_state: State::None,
//        }
//    }
//}
//
//pub fn construct_websocket_router() -> Router {
//    // this should have an Extension layer for
//    // the GameListener for the websocket
//    let ws_route = Router::new()
//        .route("/ws", get(ws_handler))
//        .layer(Extension(Arc::new(GameListener::new())));
//    ws_route
//}
//
//// should
//async fn ws_handler(
//    Extension(state): Extension(Arc<GameListener>)
//) {
//    // listen for any oncomming messages wit
//}
