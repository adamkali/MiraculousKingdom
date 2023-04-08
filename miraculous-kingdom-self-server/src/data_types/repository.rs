use mongodb::{
    bson::{
        doc,
        Document,
        oid::ObjectId,
    },
    Database,
    Collection,
    Cursor,
};
use serde::Serialize;
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


    pub async fn get_all(&mut self, mut data: Vec<T>) -> Result<(), APIError> {
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
            Err(e) => return Err(APIError::new(
                          StatusCode::INTERNAL_SERVER_ERROR,
                          e.to_string()
                      ))
        }
        Ok(())
    }

    pub async fn get_by_oid(
        &mut self, 
        mut data: T, 
        object_id: ObjectId
    ) -> Result<(), APIError> {
        let cursor = collection.find_one(
            doc! { "_id": object_id }, 
            None 
        ).await;
        match cursor {
            Ok(mut c) => {
            if let Some(result) = c {
                data = result;
            } else {
                return Err(APIError::new(
                        StatusCode::NOT_FOUND,
                        format!(
                            "Could not find object with id: {}",
                            object_id.to_hex()
                        )
                    ));
            }
        },
        Err(e) => return Err(APIError::new(
                      StatusCode::INTERNAL_SERVER_ERROR,
                      e.to_string()
                  ))
        }
        Ok(())
    }
}


pub async fn verify_id(
    id: String, 
    object_id: &mut ObjectId
) -> Result<(), APIError> {
    if let Ok(oid) = ObjectId::parse_str(&id) {
        *object_id = oid.to_owned();
        Ok(())
    } else {
        Err(APIError::new(
            StatusCode::BAD_REQUEST,
            format!(
                "{} is not a valid objectId.",
                &id
            )
        ))
    }
}

