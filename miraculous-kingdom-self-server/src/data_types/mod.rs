mod data;
mod detialed_response;
mod repository;
mod traits;
mod requests;
mod responses;
mod polling;

pub mod common {
    pub use super::data::Reward;
    pub use super::data::RewardTypes;
    pub use super::detialed_response::*;
    pub use super::requests::*;
    pub use super::repository::verify_id;
    pub use super::repository::Repository;
    pub use super::traits::*;
}

pub use data::characters;
pub use data::clock;
pub use data::engine;
pub use data::might;
pub use polling::queue;


pub async fn game_to_info(
    game: engine::Game,
    info: &mut engine::GameInfo,
) -> Result<(), common::APIError> {
    info.game_name = game.game_name;
    info.game_ruler = game.game_ruler;
    info.game_chars = game
        .game_chars
        .iter()
        .clone()
        .map(|a| a.char_name.clone())
        .collect();
    info.game_pass = game.generated_pass;
    info.game_season = game.game_season;
    Ok(())
}

pub async fn games_to_info(
    games: Vec<engine::Game>,
    infos: &mut Vec<engine::GameInfo>,
) -> Result<(), common::APIError> {
    for game in games {
        let mut info = engine::GameInfo::default();
        game_to_info(game, &mut info).await?;
        infos.push(info);
    }
    Ok(())
}
