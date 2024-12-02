pub use super::characters::{Ability, CharacterResponse, Character};
pub use super::might::MightStat;
pub use serde::{Serialize, Deserialize};
pub use utoipa::ToSchema;
pub use super::common::{MKModel, MkResponse};

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

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct RollResponse {
    pub roll_winner: CharacterResponse,
    pub rolls: Vec<RollResult>,
}

impl RollResponse {
    pub fn new(roll_winner: CharacterResponse, rolls: Vec<RollResult>) -> Self {
        Self {
            roll_winner,
            rolls,
        }
    }
}

impl Default for RollResponse {
    fn default() -> Self {
        Self {
            roll_winner: Character::new().as_response(),
            rolls: Vec::new(),
        }
    }
}
