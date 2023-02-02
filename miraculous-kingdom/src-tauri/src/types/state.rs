use serde::Deserialize;
use serde::Serialize;

use super::character::Character;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note {
    pub name: String,
    pub note: String,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clock {
    pub name: String,
    pub remaining: u8,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub id: u8,
    pub name: String,
    pub event: String, 
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub character: Character,
    pub clocks: Vec<Clock>,
    pub current_event: Event,
    pub notes: Vec<Note>,
}
