use sea_orm::entity::prelude::*;
use loco_rs::prelude::*; 
use super::_entities::classes::{self, ActiveModel, Entity, Model};
pub type Classes = Entity;


impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

impl super::_entities::classes::Model {
    pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        let class = classes::Entity::find_by_id(id)
            .one(db)
            .await?;
        class.ok_or_else(|| ModelError::EntityNotFound)

    }

    pub async fn get_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Ok(classes::Entity::find().all(db).await?)
    }
}
