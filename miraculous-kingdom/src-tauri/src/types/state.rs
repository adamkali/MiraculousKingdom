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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventType {
    #[default]
    None,
    Rolling,
    Current(Event),
    Resolving(Event),
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub character: Option<Vec<Character>>,
    pub clocks: Option<Vec<Clock>>,
    pub current_event: EventType,
    pub notes: Option<Vec<Note>>,
}

impl Default for State {
    fn default() -> Self {
        let current_event = EventType::default();

    }
}
