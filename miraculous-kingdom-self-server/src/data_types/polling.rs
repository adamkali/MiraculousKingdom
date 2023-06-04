
pub mod queue {
    use serde::{Serialize, Deserialize};
    use utoipa::ToSchema;
    use mongodb::bson::oid::ObjectId;
    use crate::data_types::{
        common::{
            MKModel, MkResponse, APIError
        },
        characters::{Ability, CharacterResponse, Clock, Character, RollTier},
        engine::{
            SeasonResponse,
            SeasonEnum,
        },
        roll_20_sided_dice
    };
    use crate::data_types::requests::{
        RollRequest, TurnRequest, RollResult
    };
    use axum::http::StatusCode;

    // a struct that holds the Ability and the Character associated
    // with it aswell as an initative number
    #[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
    pub struct QueueItem {
        pub queue_ability: Ability,
        pub queue_char: CharacterResponse,
        pub queue_initiative: i8,
        pub queue_roll: Option<RollRequest>,

    }

    #[derive(Serialize, Deserialize, Default, ToSchema, Debug, Clone)]
    pub struct Queue {
        pub id: ObjectId,
        pub game: String,
        pub turn_state: SeasonEnum,
        pub season: SeasonResponse,
        pub queue: Vec<QueueItem>,
        pub clocks: Vec<Clock>,
    }

    #[derive(Serialize, Deserialize, Default, ToSchema, Debug, Clone)]
    pub struct QueueResonse {
        pub game: String,
        pub turn_state: SeasonEnum,
        pub season: SeasonResponse,
        pub queue: Vec<QueueItem>,
        pub clocks: Vec<Clock>,
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
                clocks: Vec::<Clock>::new()
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
                clocks: queue_clone.clocks
            } 
        }
    }

    impl From<TurnRequest> for QueueItem {
        fn from(value: TurnRequest) -> Self {
            QueueItem {
                queue_char: value.character,
                queue_initiative: value.initiatve,
                queue_ability: value.ability,
                queue_roll: None,
            }
        }
    }
    
    impl QueueItem {
        pub fn new() -> Self {
            QueueItem {
                queue_char: Character::default().as_response(),
                queue_initiative: 0,
                queue_ability: Ability::default(),
                queue_roll: None,
            }
        }

        pub fn default() -> Self {
            QueueItem::new()
        }

        pub fn add_roll(&mut self, roll: RollRequest) {
            self.queue_roll = Some(roll);
        }

        pub async fn roll(&self) -> Result<Vec<RollResult>, APIError> {
            match self.queue_roll.clone() {
                Some(roll) => {
                    match self.queue_ability.ability_unlock.roll_tier {
                        RollTier::None => {
                            // all characters in the roll win the roll automatically
                            let mut results: Vec<RollResult> = Vec::with_capacity(roll.rolls.len());
                            for char in roll.rolls {
                                results.push(RollResult { roll_character: char, roll_value: 0, roll_success: true });
                            }
                            Ok(results)
                        },
                        _ => {
                            // roll the dice
                            let mut results: Vec<RollResult> = Vec::with_capacity(roll.rolls.len());
                            for char in roll.rolls {
                               let mut roll_value = roll_20_sided_dice(); 
                               roll_value += char.char_might.get_might(self.queue_ability.ability_unlock.might.clone());
                               results.push(RollResult { 
                                   roll_character: char,
                                   roll_value: roll_value as i8, 
                                   roll_success: self.queue_ability.ability_unlock.roll_tier.clone().from_roll(roll_value as i8) 
                               });
                            }
                            Ok(results)
                        }
                    }
                },
                None => Err(APIError::new(StatusCode::INTERNAL_SERVER_ERROR, "No roll found".to_string()))
            } 
        }
    }
}
