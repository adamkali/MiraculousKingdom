use crate::data_types::characters::{
    Character,
    Ability
};
use crate::data_types::common::APIError;
use crate::data_types::engine::SeasonResponse;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum Episode {
    NONE,
    ABILITYCHOOSE,
    TARGETCHOICE,
    ROLLRESULT,
}

impl ToString for Episode {
    fn to_string(&self) -> String {
        match *self {
            Self::NONE => "NONE".to_string(),
            Self::ABILITYCHOOSE => "ABILITYCHOOSE".to_string(),
            Self::TARGETCHOICE => "TARGETCHOICE".to_string(),
            Self::ROLLRESULT => "ROLLRESULT".to_string(),
        } 
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Turn {
    /// Ability for the roll resolution
    pub turn_ability: Ability,
    /// A mapping from every character secret 
    /// with there roll result 
    /// at the start it will only be 0 
    pub turn_characters: HashMap<String, i16>,
    /// isReady to move to the next episode readyness
    pub turn_ready: bool
}

#[derive(Debug, Clone)]
pub struct WSQueue {
    pub queue: Arc<[Character]>,
    pub queue_state: Episode,
    pub queue_season: Option<SeasonResponse>,
    pub queue_turn: HashMap<String, Turn>,
    pub global_broabcast: broadcast::Sender<String>,
}

impl WSQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new([]),
            queue_state: Episode::NONE,
            queue_season: None,
            queue_turn: HashMap::new(),
            global_broabcast: broadcast::channel(10).0,
        }
    }

    pub fn is_characers_ready(&self) -> bool {
        let queue_char_length = (*self).queue.len();
        let queue_turn_subscribed_length = (*self).queue_turn.len();
        if queue_char_length != queue_turn_subscribed_length { return false; }
        self.queue_turn.iter().all(|a| a.1.turn_ready )
    }

    pub async fn receive_request(&mut self, request: &impl WSRequestTrait) -> Result<(), APIError> {
        match self.queue_turn.get(
            &request.get_owner()
        ) {
            Some(turn) => { Ok(()) },
            None => {
                Err(APIError {
                    message: "Could not find the owner of the request in the Server Queue. Please try again.".to_string(),
                    status_code: 404
                })
            }
        }
    }
}

pub trait WSRequestTrait {
    fn consume_request(&self, turn: &mut Turn);
    fn get_owner(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum WSRequest {
    ABILITYREQUEST(WSAbilityRequest),
    CHARACTERREQUEST(WSTargetRequest),
    ROLLREQUEST(WSRollRequest),
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct WSAbilityRequest {
    // the person who sent the request 
    owner: String, 
    // the ability to be resolved
    ability: Ability,
}

impl WSRequestTrait for WSAbilityRequest {
    fn consume_request(&self, turn: &mut Turn) { turn.turn_ability = self.ability.clone(); }
    fn get_owner(&self) -> String { (*self.owner).to_string() }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct WSTargetRequest {
    // the person who sent the request 
    owner: String,
    // a list of character Strings to register map the 
    targets: Vec<String>,
}

impl WSRequestTrait for WSTargetRequest {
    fn consume_request(&self, turn: &mut Turn) { 
        self.targets.iter().for_each(|target| {
            turn.turn_characters.insert(target.clone(), 0);
        });
    }
    fn get_owner(&self) -> String { (*self.owner).to_string() }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct WSRollRequest {
    // the the owner of the request
    owner: String,
    // the person who sent the request
    secret: String,
    // modifiers to be applied to the roll 
    // TODO: implement modifiers
    // a reference to the ability to be rolled
    ability: Option<Ability>,
    // a reference to the character to be rolle
    character: Option<Character>,

}

impl WSRequestTrait for WSRollRequest {
    fn consume_request(&self, turn: &mut Turn) { 
        let ability = self.ability.clone();
        let character = self.character.clone();

        if ability.is_some() && character.is_some() {
            let might_req = ability.unwrap().ability_unlock;
            let might_char = character
                                .unwrap()
                                .char_might
                                .get_might(might_req.might);
            let roll = crate::data_types::roll_20_sided_dice() + might_char;

            *turn.turn_characters.get_mut(&self.owner).unwrap() = roll as i16;
        }
    }
    fn get_owner(&self) -> String { (*self.owner).to_string() }
}




