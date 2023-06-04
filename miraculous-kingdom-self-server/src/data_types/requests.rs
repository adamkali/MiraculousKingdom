pub use super::characters::{Ability, CharacterResponse};
pub use super::might::MightStat;
pub use serde::{Serialize, Deserialize};
pub use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TurnRequest {
    pub ability: Ability,
    pub character: CharacterResponse,
    pub initiatve: i8,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RollRequest {
    pub owner: String,
    pub rolls: Vec<CharacterResponse>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RollResult {
    pub roll_character: CharacterResponse,
    pub roll_value: i8,
    pub roll_success: bool,
}
