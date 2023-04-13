use super::common::APIError;
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use utoipa::ToSchema;

// Character ================================

/// This enum represents the possible states of a character in a websocket game.
/// - Waiting: the character is waiting for its turn to come. - Going: the character is currently taking its turn.
/// - Gone: the character has already taken its turn and is no longer in play.
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum CharacterState {
    #[default]
    Waiting,
    Going,
    Gone,
}

/// A struct representing a character in the game.
#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Character {
    /// The ObjectId representing the character in MongoDB.
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub game: ObjectId,
    /// A secret lock to the character.
    pub secret: String,
    /// The name of the character.
    pub char_name: String,
    /// The class of the character.
    pub char_class: Class,
    /// The character's clocks.
    pub char_clocks: Vec<Clock>,
    /// The character's might.
    pub char_might: Might,
    /// The character's abilities.
    // TODO(adamkali):
    pub abilities: Vec<Ability>,
    /// The character's state.
    pub char_state: CharacterState,
}

/// A struct representing a new character request from a client.
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct NewCharacter {
    /// A secret lock to the character for getting the character.
    pub secret: String,
    /// The name of the character.
    pub char_name: String,
    /// The class of the character. Avaliable Classes will be.
    /// WarGeneral, Aficianado, Researcher, Cardinal,
    /// SpyMaster, Diplomat, Merchant, Prince,
    pub char_class: ClassEnum,
    /// The character's might.
    pub char_might: HashMap<String, u8>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            game: ObjectId::new(),
            secret: "".to_string(),
            char_name: "No Character".to_string(),
            char_clocks: Vec::<Clock>::new(),
            char_might: Might::new_dumb(),
            abilities: Vec::<Ability>::new(),
            char_state: CharacterState::Waiting,
            char_class: Class {
                class_id: ObjectId::new(),
                class_enum: ClassEnum::WarGeneral,
                class_desc: "".to_string(),
                class_perks: "".to_string(),
                class_abilities: Vec::<Ability>::new(),
                class_name: "".to_string(),
            },
        }
    }

    /// Creates a new Character instance and associates it with a given game and character class.
    /// The character's initial state is set to `Waiting`.
    ///
    /// # Arguments
    ///
    /// * `game` - The ID of the game the character belongs to.
    /// * `character` - A `NewCharacter` instance containing information about the new character to be created.
    /// * `class` - The `Class` instance of the character's class.
    ///
    /// # Returns
    ///
    /// A new `Character` for the character and  is added to the ingame queue.
    ///
    /// # Errors
    ///
    /// Returns an `APIError` if there was an error creating the character.
    ///
    /// # Examples
    /// ```
    /// use characters::{Character, Class, NewCharacter};
    /// use mongodb::bson::oid::ObjectId;
    ///
    /// let game_id = ObjectId::new();
    /// let new_character = NewCharacter {
    ///     secret: "my-secret".to_string(),
    ///     char_name: "My Character".to_string(),
    ///     char_class: "Warrior".to_string(),
    ///     char_clocks: Vec::new(),
    ///     char_might: HashMap::new(),
    ///     abilities: Vec::new(),
    ///     char_state: CharacterState::Waiting,
    /// };
    /// let class = Class {
    ///     class_id: ObjectId::new(),
    ///     class_desc: "A warrior".to_string(),
    ///     class_perks: "Strong and tough".to_string(),
    ///     class_abilities: Vec::new(),
    ///     class_name: "Warrior".to_string(),
    /// };
    ///
    /// let character = Character::new_game(game_id, new_character, class).await.unwrap();
    /// ```
    pub async fn new_game(
        game: ObjectId,
        character: NewCharacter,
        class: Class,
    ) -> Result<Character, APIError> {
        let might: Might = Might::new(character.char_might).await?;
        Ok(Character {
            game,
            secret: character.secret,
            char_name: character.char_name,
            char_class: class.clone(),
            char_clocks: Vec::<Clock>::new(),
            char_might: might,
            abilities: class.class_abilities.iter().clone().cloned().collect(),
            char_state: CharacterState::Waiting,
        })
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

pub mod characters {
    pub use super::Ability;
    pub use super::Character;
    pub use super::CharacterState;
    pub use super::Class;
    pub use super::ClassEnum;
    pub use super::MightEnum;
    pub use super::MightRequirement;
    pub use super::NewCharacter;
    pub use super::RollTier;
}
// =========================================

// Game Class ==============================
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum State {
    #[default]
    None,
    Start,
    ClockResolution(ObjectId, ObjectId),
    ArcRolling(ObjectId),
    Episode(ObjectId, ObjectId),
    ArcResolution(ObjectId, ObjectId),
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct GameCreation {
    pub game_num_players: u16,
    pub game_name: String,
    pub game_ruler: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct GameInfo {
    pub game_name: String,
    pub game_ruler: String,
    pub game_chars: Vec<String>,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            game_name: String::new(),
            game_ruler: String::new(),
            game_chars: Vec::<String>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Game {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub game_id: ObjectId,
    pub game_chars: Vec<Character>,
    pub game_clocks: Vec<Clock>,
    pub game_state: State,
    pub game_name: String,
    pub game_ruler: String,
    pub generated_pass: String,
}

impl Game {
    pub fn new() -> Game {
        let characters: Vec<Character> = Vec::<Character>::new();
        let clocks: Vec<Clock> = Vec::<Clock>::new();
        Game {
            game_id: ObjectId::new(),
            game_chars: characters,
            game_clocks: clocks,
            game_state: State::None,
            game_name: "Not Started".to_string(),
            game_ruler: "Not Started".to_string(),
            generated_pass: "".to_string(),
        }
    }

    pub async fn start(body: GameCreation) -> Self {
        let mut ret: Game = Game::new();
        ret.game_chars = Vec::<Character>::with_capacity(body.game_num_players as usize);

        ret.game_name = body.game_name;
        ret.game_ruler = body.game_ruler;
        ret.game_clocks = Vec::<Clock>::new();

        // TODO FIx
        ret.generated_pass = ObjectId::new().to_string();

        ret.generated_pass.truncate(8);
        ret
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub mod engine {
    pub use super::Game;
    pub use super::GameCreation;
    pub use super::GameInfo;
    pub use super::State;
}
// =========================================

// Clock ===================================
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Clock {
    pub clock_duration: u8,
    pub clock_remaining: u8,
    pub clock_name: String,
    pub clock_desc: String,
    pub clock_conf: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClockPost {
    pub char_id: uuid::Uuid,
    pub clock_duration: u8,
    pub clock_remaining: u8,
    pub clock_name: String,
    pub clock_desc: String,
    pub clock_conf: bool,
}

impl Clock {
    pub fn create(post: ClockPost) -> Clock {
        Clock {
            clock_duration: post.clock_duration,
            clock_conf: post.clock_conf,
            clock_desc: post.clock_desc,
            clock_name: post.clock_name,
            clock_remaining: post.clock_remaining,
        }
    }
}

pub mod clock {
    pub use super::Clock;
    pub use super::ClockPost;
}
// =========================================

// Might ===================================
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Might {
    pub might_military: MightStat,
    pub might_culture: MightStat,
    pub might_religion: MightStat,
    pub might_science: MightStat,
    pub might_diplomacy: MightStat,
    pub might_espionage: MightStat,
}

impl Might {
    pub async fn new(stats: HashMap<String, u8>) -> Result<Might, APIError> {
        let mut might: Might = Might::new_dumb();
        let option_error: Arc<Mutex<Option<APIError>>> = Arc::new(Mutex::new(None));
        let futs: Arc<Mutex<Vec<MightStat>>> = Arc::new(Mutex::new(Vec::with_capacity(6)));
        let mut handles = Vec::new();

        for (key, value) in stats {
            let option_error_clone = option_error.clone();
            let futs_clone = futs.clone();
            let handle = tokio::spawn(async move {
                match MightEnum::create_might(key, value) {
                    Ok(m) => {
                        let mut futs = futs_clone.lock().unwrap();
                        futs.push(m);
                    }
                    Err(e) => {
                        let mut option_error = option_error_clone.lock().unwrap();
                        *option_error = Some(e.clone());
                    }
                }
            });
            handles.push(handle);
        }

        // Await all the spawned tasks before proceeding
        for handle in handles {
            if let Err(e) = handle.await {
                return Err(APIError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                ));
            }
        }

        let option_error = option_error.lock().unwrap();
        if let Some(e) = option_error.as_ref() {
            return Err(e.clone());
        }
        let futs = futs.lock().unwrap();
        for f in futs.iter() {
            match f.stat_enum {
                MightEnum::Military => might.might_military = (*f).clone(),
                MightEnum::Culture => might.might_culture = (*f).clone(),
                MightEnum::Religion => might.might_religion = (*f).clone(),
                MightEnum::Science => might.might_science = (*f).clone(),
                MightEnum::Diplomacy => might.might_diplomacy = (*f).clone(),
                MightEnum::Espionage => might.might_espionage = (*f).clone(),
            }
        }

        Ok(might)
    }

    pub fn new_dumb() -> Might {
        Might {
            might_military: MightStat {
                stat_enum: MightEnum::Military,
                stat_name: "military".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },
            might_culture: MightStat {
                stat_enum: MightEnum::Culture,
                stat_name: "culture".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },

            might_religion: MightStat {
                stat_enum: MightEnum::Religion,
                stat_name: "religion".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },
            might_science: MightStat {
                stat_enum: MightEnum::Science,
                stat_name: "science".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },
            might_diplomacy: MightStat {
                stat_enum: MightEnum::Diplomacy,
                stat_name: "diplomacy".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },
            might_espionage: MightStat {
                stat_enum: MightEnum::Espionage,
                stat_name: "espionage".to_string(),
                stat_value: -2_i16,
                stat_exp: 0,
                stat_token: 0,
            },
        }
    }
}

pub static MIGHTMAXEXP: u8 = 5_u8;

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct MightStat {
    pub stat_enum: MightEnum,
    pub stat_name: String,
    pub stat_value: i16,
    pub stat_exp: u8,
    pub stat_token: u8,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum MightEnum {
    #[default]
    Military,
    Culture,
    Religion,
    Science,
    Diplomacy,
    Espionage,
}

impl MightEnum {
    pub fn determine(value: String) -> Result<MightEnum, APIError> {
        if value == *"Military".to_string() {
            Ok(MightEnum::Military)
        } else if value == *"Culture".to_string() {
            Ok(MightEnum::Culture)
        } else if value == *"Science".to_string() {
            Ok(MightEnum::Science)
        } else if value == *"Religion".to_string() {
            Ok(MightEnum::Religion)
        } else if value == *"Diplomacy".to_string() {
            Ok(MightEnum::Diplomacy)
        } else if value == *"Espionage".to_string() {
            Ok(MightEnum::Espionage)
        } else {
            Err(APIError::new(
                axum::http::StatusCode::BAD_REQUEST,
                format!("{} is not a known Might", value),
            ))
        }
    }

    pub fn create_might(name: String, value: u8) -> Result<MightStat, APIError> {
        let type_of: MightEnum = MightEnum::determine(name.clone())?;
        Ok(MightStat {
            stat_enum: type_of,
            stat_name: name,
            stat_value: value as i16 - 2_i16,
            stat_exp: 0,
            stat_token: 0,
        })
    }
}

pub mod might {
    pub use super::Might;
    pub use super::MightStat;
    pub use super::MIGHTMAXEXP;
}

// =========================================

// Ability =================================
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Ability {
    pub ability_name: String,
    pub ability_desc: String,
    pub ability_clock: Option<Clock>,
    pub ability_unlock: MightRequirement,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum RollTier {
    #[default]
    None, // "" automatically works
    Fail,      // 1
    Bad,       // 2-6
    Neutral,   // 7-11
    Good,      // 12-16
    Great,     // 17+
    Fantastic, // Roll 20 on the dice
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct MightRequirement {
    might: MightEnum,
    roll_tier: RollTier,
    unlock: u8,
}

// =========================================

// Class ===================================
#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Class {
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
    pub class_id: mongodb::bson::oid::ObjectId,
    pub class_enum: ClassEnum,
    pub class_desc: String,
    pub class_perks: String,
    pub class_abilities: Vec<Ability>,
    pub class_name: String,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum ClassEnum {
    #[default]
    WarGeneral,
    Aficianado,
    Researcher,
    Cardinal,
    SpyMaster,
    Diplomat,
    Merchant,
    Prince,
}

impl ToString for ClassEnum {
    fn to_string(&self) -> String {
        match *self {
            Self::Aficianado => "Aficianado".to_string(),
            Self::WarGeneral => "WarGeneral".to_string(),
            _ => "Not Implemented".to_string(),
        }
    }
}

impl Class {
    pub fn new() -> Class {
        Class {
            class_id: ObjectId::new(),
            class_enum: ClassEnum::WarGeneral,
            class_desc: "".to_string(),
            class_perks: "".to_string(),
            class_abilities: Vec::<Ability>::new(),
            class_name: "".to_string(),
        }
    }
}

impl Default for Class {
    // TODO(adam): make a better default
    fn default() -> Self {
        Class::new()
    }
}

// =========================================

// Event ===================================
#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Season {
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
    pub event_id: ObjectId,
    /// The amount of turns the Character to the left of the ruler must take in order to go to
    /// the new season.
    pub event_length: u16,
    pub event_name: String,
    pub event_desc: String,
    /// Can be any type of struct that takes the Reward trait.
    // TODO(adamkali): add in this functionality with Character.
    pub event_reward: RewardTypes,
}

pub trait Reward {
    fn grant_reward(&self, character: &mut Character) -> Result<(), APIError>;
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum RewardTypes {
    #[default]
    None,
    Ability(Ability),
    Experience(u8),
}
