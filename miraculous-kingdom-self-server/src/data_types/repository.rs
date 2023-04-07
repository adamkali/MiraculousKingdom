use mongodb::{
    bson::{
        Document,
        oid::ObjectId,
    },
    Database,
    Collection,
};
use serde::Serialize;
use utoipa::ToSchema;
use super::common::APIError;
use axum::http::StatusCode;
use futures::stream::TryStreamExt;
use std::marker::PhantomData;
    
pub struct Repository<T> {
    pub model: T,
    pub collection: Collection<T>,
    phantom: PhantomData<T>,
}

impl<T: Default + Clone + Serialize> Repository<T> {
    pub fn new(db: &Database, coll_name: &str) -> Self {
        return Repository {
            model: T::default(),
            collection: db.clone().collection::<T>(coll_name),
            phantom: PhantomData
        }
    }

    pub async fn get_all(&mut self, &mut data: Vec<T>) -> Result<(), APIError> {
        let cursor = self.collection.find(None, None).await;
        match cursor {
            Ok(mut c) => {
                while let Ok(res) = c.try_next().await {
                    match res {
                        Some(r) => {
                            data.push(r)
                        },
                        None => {
                            break;
                        },
                    }
                };
            },
            Err(e) => Err(APIError::new(
                          StatusCode::INTERNAL_SERVER_ERROR,
                          e.to_string()
                      ))
        }
        Ok(())
    }
    pub async fn get_by_oid(&mut self, &mut data: T)
}
