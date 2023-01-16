use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub class_id: String,
    pub class_name: String,
    pub class_bonus: String,
    pub class_actions: Vec<ClassAction>,
    pub class_victory: ClassVictory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassAction {
    pub class_action_name: String,
    pub class_action_desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassVictory {
    pub class_victory_name: String,
    pub class_victory_prep: String,
    pub class_victory_roll: String,
    pub class_victory_resl: ClassVictoryResl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassVictoryResl {
    pub class_victory_resl_success: String,
    pub class_victory_resl_faliure: String,
}

