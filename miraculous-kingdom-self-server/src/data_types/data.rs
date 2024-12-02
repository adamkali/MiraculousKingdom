use super::common::APIError;
use super::traits::*;
use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use utoipa::ToSchema;
use rand::{thread_rng, Rng, seq::SliceRandom};

// Character ================================

    #[default]
    Waiting,
    RollingSeason,
    PlayingTurn,
    Resolve,
}

/// A struct representing a character in the game.
#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Character {
    /// The ObjectId representing the character in MongoDB.
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
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
    pub char_deck: Vec<Ability>,
    pub char_hand: Vec<Ability>,
    pub char_discard: Vec<Ability>,
    /// The character's state.
    pub char_state: CharacterState,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct CharacterResponse {
    /// A secret lock to the character.
    pub secret: String,
    /// The name of the character.
    pub char_name: String,
    /// The class of the character.
    pub char_class: ClassEnum,
    /// The character's clocks.
    pub char_clocks: Vec<Clock>,
    /// The character's might.
    pub char_might: Might,
    /// The character's abilities.
    // TODO(adamkali):
    pub char_deck: Vec<Ability>,
    pub char_hand: Vec<Ability>,
    pub char_discard: Vec<Ability>,
    /// The character's state.
    pub char_state: CharacterState,
}

impl MkResponse for CharacterResponse {}

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
    pub char_might: HashMap<MightEnum, u8>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            game: ObjectId::new(),
            secret: "".to_string(),
            char_name: "No Character".to_string(),
            char_clocks: Vec::<Clock>::new(),
            char_might: Might::new_dumb(),
            char_deck: Vec::<Ability>::new(),
            char_hand: Vec::<Ability>::new(),
            char_discard: Vec::<Ability>::new(),
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
            char_deck: class.class_abilities.iter().clone().cloned().collect(),
            char_hand: Vec::<Ability>::new(),
            char_discard: Vec::<Ability>::new(),
            char_state: CharacterState::Waiting,
        })
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new()
    }
}

impl MKModel for Character {
    type Response = CharacterResponse;
    fn as_response(&self) -> Self::Response {
        CharacterResponse {
            secret: self.secret.clone(),
            char_name: self.char_name.clone(),
            char_class: self.char_class.clone().as_response().class_enum,
            char_clocks: self.char_clocks.clone(),
            char_might: self.char_might.clone(),
            char_discard: self.char_discard.clone(),
            char_hand: self.char_hand.clone(),
            char_deck: self.char_deck.clone(),
            char_state: self.char_state.clone(),
        }
    }
}

pub mod characters {
    pub use super::Ability;
    pub use super::AbilityModel;
    pub use super::Character;
    pub use super::CharacterResponse;
    pub use super::CharacterState;
    pub use super::Class;
    pub use super::ClassEnum;
    pub use super::ClassResponse;
    pub use super::Clock;
    pub use super::DrawCard;
    pub use super::Experience;
    pub use super::MightEnum;
    pub use super::MightRequirement;
    pub use super::NewCharacter;
    pub use super::PayToken;
    pub use super::RollTier;
    pub use super::Token;
}
// =========================================

// Game Class ==============================
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
    pub game_pass: String,
    pub game_season: SeasonEnum,
}

impl Default for GameInfo {
    fn default() -> Self {
        Self {
            game_name: String::new(),
            game_ruler: String::new(),
            game_chars: Vec::<String>::new(),
            game_pass: "0000AAAA".to_string(),
            game_season: SeasonEnum::None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Game {
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
    pub game_id: ObjectId,
    pub game_chars: Vec<Character>,
    pub game_clocks: Vec<Clock>,
    pub game_name: String,
    pub game_ruler: String,
    pub generated_pass: String,
    pub game_season: SeasonEnum
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct GameResponse {
    pub game_chars: Vec<CharacterResponse>,
    pub game_clocks: Vec<Clock>,
    pub game_name: String,
    pub game_ruler: String,
    pub generated_pass: String,
    pub game_season: SeasonEnum,
}

impl MkResponse for GameResponse {}

impl MKModel for Game {
    type Response = GameResponse;
    fn as_response(&self) -> Self::Response {
        let game_chars: Vec<CharacterResponse> = Vec::new();
        self.game_chars.iter().for_each(|a| {
            a.clone().as_response();
        });
        Self::Response {
            game_chars,
            game_clocks: self.game_clocks.clone(),
            game_name: self.game_name.clone(),
            game_ruler: self.game_ruler.clone(),
            generated_pass: self.generated_pass.clone(),
            game_season: self.game_season.clone(),
        }
    }
}

impl Game {
    pub fn new() -> Game {
        let characters: Vec<Character> = Vec::<Character>::new();
        let clocks: Vec<Clock> = Vec::<Clock>::new();
        Game {
            game_id: ObjectId::new(),
            game_chars: characters,
            game_clocks: clocks,
            game_name: "Not Started".to_string(),
            game_ruler: "Not Started".to_string(),
            generated_pass: "".to_string(),
            game_season: SeasonEnum::None,
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
        ret.game_season = SeasonEnum::None;
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
    pub use super::GameResponse;
    pub use super::Season;
    pub use super::SeasonResponse;
    pub use super::SeasonEnum;
    pub use super::RewardTypes;
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
    pub fn get_might(&self, might_enum: MightEnum) -> u8 {
        match might_enum {
            MightEnum::Military => self.might_military.stat_value as u8,
            MightEnum::Culture => self.might_culture.stat_value as u8,
            MightEnum::Religion => self.might_religion.stat_value as u8,
            MightEnum::Science => self.might_science.stat_value as u8,
            MightEnum::Diplomacy => self.might_diplomacy.stat_value as u8,
            MightEnum::Espionage => self.might_espionage.stat_value as u8,
            MightEnum::None => 0,
        }
    }


    pub async fn new(stats: HashMap<MightEnum, u8>) -> Result<Might, APIError> {
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
                _ => { unreachable!("Invalid MightEnum: {:?}", f.stat_enum) }
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
    pub stat_exp: i8,
    pub stat_token: i8,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug, Eq, PartialEq, Hash)]
pub enum MightEnum {
    #[default]
    None,
    Military,
    Culture,
    Religion,
    Science,
    Diplomacy,
    Espionage,
}

impl MightEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Espionage => "Espionage",
            Self::Military => "Military",
            Self::Culture => "Culture",
            Self::Science => "Science",
            Self::Religion => "Religion",
            Self::Diplomacy => "Diplomacy",
        }
    }

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

    pub fn create_might(name: MightEnum, value: u8) -> Result<MightStat, APIError> {
        Ok(MightStat {
            stat_enum: name.clone(),
            stat_name: name.as_str().to_string(),
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
    pub ability_rewards: Vec<RewardTypes>,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct AbilityModel {
    pub ability_name: String,
    pub ability_desc: String,
    pub ability_clock: Option<Clock>,
    pub ability_unlock: MightRequirement,
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
    pub id: ObjectId,
    pub ability_reward: Vec<RewardTypes>,
}

impl AbilityModel {
    pub fn new() -> Self {
        AbilityModel {
            ability_name: "".to_string(),
            ability_desc: "".to_string(),
            ability_clock: None,
            ability_unlock: MightRequirement::default(),
            id: ObjectId::new(),
            ability_reward: Vec::new(),
        }
    }
}

impl MkResponse for Ability {}

impl MKModel for AbilityModel {
    type Response = Ability;
    fn as_response(&self) -> Self::Response {
        Ability {
            ability_name: self.ability_name.clone(),
            ability_desc: self.ability_desc.clone(),
            ability_clock: self.ability_clock.clone(),
            ability_unlock: self.ability_unlock.clone(),
            ability_rewards: self.ability_reward.clone(),
        }
    }
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

impl RollTier {
    pub fn from_roll(self, roll: i8) -> bool {
        match self {
            RollTier::None => true,
            RollTier::Fail => {
                if roll <= 1 {
                    true
                } else {
                    false
                }
            },
            RollTier::Bad => {
                if roll > 2 {
                    true  
                } else {
                    false
                }
            },
            RollTier::Neutral => {
                if roll > 6 {
                    true
                } else {
                    false
                }
            }, 
            RollTier::Good => {
                if roll > 11 {
                    true
                } else {
                    false
                }
            },
            RollTier::Great => {
                if roll > 16 {
                    true
                } else {
                    false
                }
            },
            RollTier::Fantastic => {
                if roll >= 20 {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct MightRequirement {
    pub might: MightEnum,
    pub roll_tier: RollTier,
    pub unlock: u8,
}

// =========================================

// Class ===================================
#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Class {
    #[serde(
        serialize_with = "serialize_object_id_as_hex_string",
        skip_deserializing
    )]
    pub class_id: ObjectId,
    pub class_enum: ClassEnum,
    pub class_desc: String,
    pub class_perks: String,
    pub class_abilities: Vec<Ability>,
    pub class_name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct ClassResponse {
    pub class_enum: ClassEnum,
    pub class_desc: String,
    pub class_perks: String,
    pub class_abilities: Vec<Ability>,
    pub class_name: String,
}

impl MkResponse for ClassResponse {}

impl MKModel for Class {
    type Response = ClassResponse;
    fn as_response(&self) -> Self::Response {
        Self::Response {
            class_enum: self.class_enum.clone(),
            class_desc: self.class_desc.clone(),
            class_perks: self.class_perks.clone(),
            class_abilities: self.class_abilities.clone(),
            class_name: self.class_name.clone(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum ClassEnum {
    #[default]
    WarGeneral,
    Aficianado,
    Scientist,
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
            Self::Scientist => "Scientist".to_string(),
            Self::SpyMaster => "SpyMaster".to_string(),
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

impl Season {
    pub fn new() -> Season {
        Season {
            event_id: ObjectId::new(),
            event_length: 0,
            event_name: "".to_string(),
            event_desc: "".to_string(),
            event_reward: RewardTypes::None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct SeasonResponse {
    pub event_length: u16,
    pub event_name: String,
    pub event_desc: String,
    pub event_reward: RewardTypes,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum SeasonEnum {
    #[default]
    None,
    SeasonResponse(SeasonResponse),
}

impl MkResponse for SeasonResponse {}

impl MKModel for Season {
    type Response = SeasonResponse;
    fn as_response(&self) -> Self::Response {
        Self::Response {
            event_length: self.event_length,
            event_name: self.event_name.clone(),
            event_desc: self.event_desc.clone(),
            event_reward: self.event_reward.clone(),
        }
    }
}

pub trait Reward {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError>;
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub enum RewardTypes {
    #[default]
    None,
    Ability(Ability),
    Experience(Experience),
    Clock(Clock),
    Token(Token),
    DrawCard(DrawCard),
    PayToken(PayToken),
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Token {
    pub token_type: MightEnum,
    pub token_amount: i8,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct PayToken {
    pub token_type: MightEnum,
    pub token_amount: i8,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Experience {
    pub exp_type: MightEnum,
    pub exp_amount: i8,
}

#[derive(Default, Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct DrawCard {
    pub amount: u8,
}


impl Reward for Token {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        match self.token_type {
            MightEnum::Military => character.char_might.might_military.stat_token += self.token_amount,
            MightEnum::Culture => character.char_might.might_culture.stat_token += self.token_amount,
            MightEnum::Science => character.char_might.might_science.stat_token += self.token_amount,
            MightEnum::Religion => character.char_might.might_religion.stat_token += self.token_amount,
            MightEnum::Diplomacy => character.char_might.might_diplomacy.stat_token += self.token_amount,
            MightEnum::Espionage => character.char_might.might_espionage.stat_token += self.token_amount,
            MightEnum::None => {}
        }
        Ok(())
    }
}

impl Reward for Clock {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        character.char_clocks.push(self.clone());
        Ok(())
    }
}

impl Reward for Experience {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        match self.exp_type {
            MightEnum::Military =>   character.char_might.might_military.stat_exp += self.exp_amount,
            MightEnum::Culture =>     character.char_might.might_culture.stat_exp += self.exp_amount,
            MightEnum::Science =>     character.char_might.might_science.stat_exp += self.exp_amount,
            MightEnum::Religion =>   character.char_might.might_religion.stat_exp += self.exp_amount,
            MightEnum::Diplomacy => character.char_might.might_diplomacy.stat_exp += self.exp_amount,
            MightEnum::Espionage => character.char_might.might_espionage.stat_exp += self.exp_amount,
            MightEnum::None => {}
        }
        Ok(())
    }
}

impl Reward for Ability {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        character.char_discard.push(self.clone());
        Ok(())
    }
}

impl Reward for DrawCard {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        if character.char_deck.is_empty() {
            // Take the discard pile and shuffle it into the char_deck
            character.char_deck = character.char_discard.clone();
            character.char_discard.clear();
            
            // Shuffle the deck using the thread_rng and SliceRandom trait
            let mut rng = thread_rng();
            character.char_deck.shuffle(&mut rng);
        }
        for _ in 0..self.amount {
            let card = character.char_deck.pop().unwrap();
            character.char_hand.push(card);
        }

        Ok(())
    }
}

impl Reward for PayToken {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        match self.token_type {
            MightEnum::Military => 
                character.char_might.might_military.stat_token -= self.token_amount,
            MightEnum::Culture => 
                character.char_might.might_culture.stat_token -= self.token_amount,
            MightEnum::Science => 
                character.char_might.might_science.stat_token -= self.token_amount,
            MightEnum::Religion => 
                character.char_might.might_religion.stat_token -= self.token_amount,
            MightEnum::Diplomacy => 
                character.char_might.might_diplomacy.stat_token -= self.token_amount,
            MightEnum::Espionage => 
                character.char_might.might_espionage.stat_token -= self.token_amount,
            MightEnum::None => return Err(APIError::new(
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid MightEnum".to_string(),
                    )),
        }
        Ok(())
    }
}

impl Reward for RewardTypes {
    fn grant_reward(&self, character: &mut CharacterResponse) -> Result<(), APIError> {
        match self {
            RewardTypes::Ability(ability) => ability.grant_reward(character),
            RewardTypes::DrawCard(draw_card) => draw_card.grant_reward(character),
            RewardTypes::Clock(clock) => clock.grant_reward(character),
            RewardTypes::Token(team) => team.grant_reward(character),
            RewardTypes::PayToken(token) => token.grant_reward(character),
            RewardTypes::Experience(exp) => exp.grant_reward(character),
            RewardTypes::None => Ok(()),
        }
    }
}
