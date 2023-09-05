use crate::MiraculousKingdomDatabaseConfig;
use crate::utils::FromRawModel;
use crate::{data::ability::Ability, utils::get_many_by_ids};
use crate::data::common::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClassRawModel {
    pub class_id: String,
    pub class_enum: String,
    pub class_desc: String,
    pub class_perks: String,
    pub class_abilities: Vec<String>,
    pub class_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Class {
    pub class_id: String,
    pub class_enum: ClassEnum,
    pub class_desc: String,
    pub class_perks: String,
    pub class_abilities: Vec<Ability>,
    pub class_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
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

impl From<String> for ClassEnum {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Aficianado" => Self::Aficianado,
            "WarGeneral" => Self::WarGeneral,
            "Scientist" => Self::Scientist,
            "SpyMaster" => Self::SpyMaster,
            _ => Self::WarGeneral,
        }
    }
} 

#[axum::async_trait]
impl FromRawModel<Class> for ClassRawModel {
    async fn resolve(
        &self,
        config: &MiraculousKingdomDatabaseConfig,
    ) -> Result<Class, MKRepositoryError> {

        let abilities = get_many_by_ids::<Ability>(
            config,
            "ability",
            self.class_abilities
                .iter()
                .map(|a| a.as_str())
                .collect::<Vec<&str>>()
                .as_slice()
        ).await?;

        Ok(Class {
            class_id: self.class_id.clone(),
            class_enum: self.class_enum.clone().into(),
            class_desc: self.class_desc.clone(),
            class_perks: self.class_perks.clone(),
            class_abilities: abilities,
            class_name: self.class_name.clone(),
        })
    }
}

fn special_from(v: Vec<String>) -> String {
    let mut html = "".to_string();
    v.iter()
        .for_each(|a| {
            html.push_str(a.clone().as_str())
        });
    html
}

impl From<Class> for String {
    fn from(value: Class) -> Self {

        let mut abilities: Vec<String> = Vec::new();

        value.class_abilities
            .iter()
            .for_each(|a| {
                abilities.push(Into::<String>::into(a.clone()));
            });
    
        // chunck the abilities array into 6 sets of three abilities
        let mut ability_matrix: Vec<Vec<String>> = Vec::with_capacity(6);

        // chunck me daddy
        for _ in [0..abilities.len()] { // should be 18
            let mut v: Vec<String> = Vec::with_capacity(3);
            for _ in [0..3] {
                v.push(abilities.pop().unwrap());
            }
            ability_matrix.push(v);
        }

        format!(
r#"
<div class="group relative h-[60rem] w-[64rem] mb-4">
    <div
        class="absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
    />
        <div
            class="mx-2 flex h-full from-fuchsia-500"
        >
            Class Name: {}
        </div>
        <div
            class="mx-2 flex h-full from-blue-300"
        >
            <span class="text-2xl text-fuchsia-600">Class Description</span>
            <p class="mb-2">
                {}
            </p>
        </div>
        <div
            class="mx-2 flex h-full from-blue-300"
        >
            <span class="text-2xl text-fuchsia-600">Class Perks</span>
            <p class="mb-2">
                {}
            </p>
        </div>
        <div
            class="flex flex-1"
        >
            <span class="text-2xl text-fuchsia-600">Class Abilities</span>
            <div
                class="mx-2 grid grid-rows-6 gap-4"
            >
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
                <div
                    class="mx-2 grid grid-cols-3 gap-4""
                >
                    {}
                </div>
            </div>
        </div>
    </div>
</div>
"#,
            value.class_name,
            value.class_desc,
            value.class_perks,
            special_from(ability_matrix.get(0).unwrap().clone()),
            special_from(ability_matrix.get(1).unwrap().clone()),
            special_from(ability_matrix.get(2).unwrap().clone()),
            special_from(ability_matrix.get(3).unwrap().clone()),
            special_from(ability_matrix.get(4).unwrap().clone()),
            special_from(ability_matrix.get(5).unwrap().clone()),
        )
    }
}
