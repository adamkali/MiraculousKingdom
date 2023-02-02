#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    #[default]
    None,
    Reaction,
    Combo,
    Clock,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ability {
    pub name: String,
    pub desc: String,
    pub tag: Tag,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub name: String,
    pub desc: String,
    pub expg: String,
    pub perk: String,
    pub abls: Vec<Ability>,
}
