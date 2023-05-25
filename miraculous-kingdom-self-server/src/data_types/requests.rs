pub use super::characters::{Ability, CharacterResponse};
pub use serde::{Serialize, Deserialize};
pub use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct TurnRequest {
    pub ability: Ability,
    pub character: CharacterResponse,
    pub initiatve: u8,
}

pub enum QueueWebsoketMessageEnum {
    GetQueue,
    SubmitTurnRequest(TurnRequest),
    GetSeason,
    RollSeason(u16),
}
