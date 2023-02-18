use std::fmt::Display;

pub use super::class::Class;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Might {
    pub name: String,
    pub token: String,
    pub bonus: u8,
    pub tokens: u8,
}

pub struct MightRequestItem {
    pub name: String,
    pub roll: u8,
}

pub type MightRequest = Vec<MightRequestItem>;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
    pub name: String,
    pub value: u8,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Character {
    pub name: String,
    pub class: Class,
    pub ruler: String,
    pub country: String,
    pub mights: Vec<Might>,
    pub kings_favor: Resource,
    pub sabatoge: Resource,
    pub strategem: Resource
}

impl Character {
    pub fn mk_character_create()
}
