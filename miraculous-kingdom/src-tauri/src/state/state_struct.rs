use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub initialized:    bool,
    pub characters:     Vec<Character>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name:                   String,
    pub class:                  String,
    pub class_id:               String,
    pub country:                String,
    pub sovreign:               String,
    pub military_might:         u8, 
    pub cultural_might:         u8,
    pub scientific_might:       u8,
    pub diplomatic_might:       u8,
    pub religious_might:        u8,
    pub criminal_might:         u8,
    pub military_token:         u8,
    pub gold_token:             u8,
    pub citizen_token:          u8,
    pub technological_token:    u8,
    pub cultural_token:         u8,
    pub religion_token:         u8,
    pub kings_favor:            u8,
    pub strategem:              u8,
    pub sabatoge:               u8,
    pub notes:                  Vec<String>,
}

impl Default for Character {
    fn default() -> Self {
        Self { 
            name:                   "Choose a Name".to_string(), 
            class:                  "Choose a Class".to_string(), 
            class_id:               "000".to_string(), 
            country:                "Choose a Country Name for the Group".to_string(), 
            sovreign:               "Choose your group's sovriegn".to_string(), 
            military_might:         2_u8,
            cultural_might:         2_u8,
            scientific_might:       2_u8,
            diplomatic_might:       2_u8,
            religious_might:        2_u8,
            criminal_might:         2_u8,
            military_token:         0_u8, 
            gold_token:             0_u8, 
            technological_token:    0_u8, 
            cultural_token:         0_u8, 
            religion_token:         0_u8, 
            citizen_token:          0_u8,
            kings_favor:            0_u8, 
            strategem:              0_u8, 
            sabatoge:               0_u8, 
            notes:                  Vec::new() 
        }
    }
}

impl Default for State {
   fn default() -> Self {
        Self { 
            initialized: false, 
            characters: vec![Character::default()] 
        }
   }
}

impl State {
    pub fn 
}

