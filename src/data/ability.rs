use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Ability {
    pub id: String,
    pub ability_name: String,
    pub ability_desc: String,
    pub ability_clock: Option<Clock>,
    pub ability_unlock: MightRequirement,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct AbilityModel {
    pub id: String,
    pub ability_name: String,
    pub ability_desc: String,
    pub ability_clock: Option<Clock>,
    pub ability_unlock: MightRequirement,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
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

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MightRequirement {
    might: MightEnum,
    roll_tier: RollTier,
    unlock: u8,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Clock {
    pub clock_duration: u8,
    pub clock_remaining: u8,
    pub clock_name: String,
    pub clock_desc: String,
    pub clock_conf: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
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

impl Display for MightEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Automatic"),
            Self::Military => write!(f, "Military"),
            Self::Culture => write!(f, "Culture"),
            Self::Science => write!(f, "Science"),
            Self::Religion => write!(f, "Religion"),
            Self::Diplomacy => write!(f, "Diplomacy"),
            Self::Espionage => write!(f, "Espionage"),
        }
    }
}

impl Display for RollTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Automatic"),
            Self::Fail => write!(f, "Fail"),
            Self::Bad => write!(f, "Bad"),
            Self::Neutral => write!(f, "Neutral"),
            Self::Good => write!(f, "Good"),
            Self::Great => write!(f, "Great"),
            Self::Fantastic => write!(f, "Fantastic"),
        }
    }
}

impl From<AbilityModel> for Ability {
    fn from(value: AbilityModel) -> Self {
        let mut ability: Ability = Ability::new();
        ability.ability_name = value.ability_name;
        ability.ability_desc = value.ability_desc;
        ability.ability_clock = value.ability_clock;
        ability.ability_unlock = value.ability_unlock;
        ability
    }
}

impl From<Ability> for String {
    fn from(value: Ability) -> Self {
        format!(
r#"
<div class="group relative h-[20rem] w-[24rem] mb-4">
    <div class="absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"></div>
    <div
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-black px-4 py-2 text-justify text-sm backdrop-blur to-blue-200 z-10"
    >
        <div class="mb-2 text-2xl text-fuchsia-600">
            {}
        </div>
        <p class="mb-2">
            {}
        </p>
        {}
        <div class="place-items-end">
            <div class="flex flex-row text-lg">
                {}
            </div>
                <div
                    class="place-items-end rounded-xl bg-white px-2 py-1 text-center text-black"
                >
                    {}
                </div>
            </div>
        </div>
    </div>
</div>
"#,
            value.ability_name,
            value.ability_desc,
            {
                // TODO: Implement better Clock logic
                if let Some(clock) = value.ability_clock {
                    clock.clock_name
                } else { "".to_string() }
            } as String,
            // TODO: Implement MightRequirement Render logic
            value.ability_unlock.might,
            value.ability_unlock.roll_tier
        )
    }
}

impl Ability {
    pub fn new() -> Self {
        Self {
            id: "ability:none".to_string(),
            ability_name: "".to_string(),
            ability_desc: "".to_string(),
            ability_clock: None,
            ability_unlock: MightRequirement::default(),
        }
    }
}
