use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use super::{
    characters::{Ability, CharacterResponse},
    engine::SeasonResponse,
    traits::{MKModel, MkResponse},
};
use mongodb::bson::oid::ObjectId;

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum Status {
    #[default]
    None,
    Rolling,
    AbilityColl,
    Resolve,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct QueueItem {
    pub item_initiative: i8,
    pub item_ability: Ability,
    pub item_character: CharacterResponse
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Queue {
    pub status: Status,
    pub season: SeasonResponse,
    pub game: String,
    pub queue_items: Vec<QueueItem>
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct QueueModel {
    pub id:  ObjectId,
    pub status: Status,
    pub season: SeasonResponse,
    pub game: String,
    pub queue_items: Vec<QueueItem>
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum WebsocketMessage {
    Start(String),
    #[default]
    GetQueue,
    ClearQueue,
    GetSeason,
    RollSeason,
    SendQueueItem(QueueItem),
    End
}

impl MkResponse for Queue { }

impl MKModel for QueueModel {
    type Response = Queue;
    fn as_response(&self) -> Self::Response {
        // copy all data from self to Response exclude ObjectId
        Queue {
            status: self.status.clone(),
            season: self.season.clone(),
            game: self.game.clone(),
            queue_items: self.queue_items.clone()
        }
    }
}

pub mod websockets {
    pub use super::Queue;
    pub use super::QueueModel;
    pub use super::QueueItem;
    pub use super::Status;
    pub use super::WebsocketMessage;
}
