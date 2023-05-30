
pub mod queue {
    use serde::{Serialize, Deserialize};
    use utoipa::ToSchema;
    use mongodb::bson::oid::ObjectId;
    use crate::data_types::{
        common::{
            MKModel, MkResponse,
        },
        characters::{Ability, CharacterResponse},
        engine::{
            SeasonResponse,
            SeasonEnum,
        }
    };
    use crate::data_types::requests::TurnRequest;

    // a struct that holds the Ability and the Character associated
    // with it aswell as an initative number
    #[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
    pub struct QueueItem {
        pub queue_ability: Ability,
        pub queue_char: CharacterResponse,
        pub queue_initiative: i8,
    }

    #[derive(Serialize, Deserialize, Default, ToSchema, Debug, Clone)]
    pub struct Queue {
        pub id: ObjectId,
        pub game: String,
        pub turn_state: SeasonEnum,
        pub season: SeasonResponse,
        pub queue: Vec<QueueItem>,
        pub status: bool
    }

    #[derive(Serialize, Deserialize, Default, ToSchema, Debug, Clone)]
    pub struct QueueResonse {
        pub game: String,
        pub turn_state: SeasonEnum,
        pub season: SeasonResponse,
        pub queue: Vec<QueueItem>,
    }

    impl Queue {
        // function to the Queue.queue by Queue.queue.queue_initiative
        pub fn sort_queue(&mut self)  {
            self.queue.sort_by(|a, b| b.queue_initiative.cmp(&a.queue_initiative));
        }

        pub fn new() -> Self {
            Queue {
                id: ObjectId::new(),
                game: String::new(),
                turn_state: SeasonEnum::None,
                season: SeasonResponse::default(),
                queue: Vec::<QueueItem>::new(),
                status: false,
            }
        }

        pub fn default() -> Self {
            Queue::new()
        }

        pub fn push_queue_item(&mut self, item: QueueItem) {
            self.queue.push(item);
        }
    }

    impl MkResponse for QueueResonse { }

    impl MKModel for Queue {
        type Response = QueueResonse;
        fn as_response(& self) -> Self::Response {
            let mut queue_clone = self.clone();
            queue_clone.sort_queue();
            QueueResonse {
                game: queue_clone.game,
                turn_state: queue_clone.turn_state,
                season: queue_clone.season,
                queue: queue_clone.queue,
            } 
        }
    }

    impl From<TurnRequest> for QueueItem {
        fn from(value: TurnRequest) -> Self {
            QueueItem {
                queue_char: value.character,
                queue_initiative: value.initiatve,
                queue_ability: value.ability,
            }
        }
    }
}
