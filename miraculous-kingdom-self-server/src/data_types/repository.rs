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
use serde::{Serialize, de::DeserializeOwned};
use super::common::APIError;
use axum::http::StatusCode;
use futures::stream::TryStreamExt;
use std::marker::PhantomData;


#[derive(Clone)]
pub struct Repository<
    T: Default + Clone + Serialize 
     + Sync + Send + Unpin 
     + DeserializeOwned
> {
    pub model: T,
    pub collection: Collection<T>,
    phantom: PhantomData<T> ,
}


impl<
    T: Default + Clone + Serialize 
     + Sync + Send + Unpin 
     + DeserializeOwned
> Repository<T> {
    pub fn new(db: &Database, coll_name: &str) -> Self {
        return Repository {
            model: T::default(),
            collection: db.clone().collection::<T>(coll_name),
            phantom: PhantomData,
        }
    }


    pub async fn get_all(
        &mut self, 
         mut data: Vec<T>
    ) -> Result<(), APIError> {
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

    pub async fn get_all_with_filter(
       &mut self,
       mut data: Vec<T>,
       filter: Document,
    ) -> Result<(), APIError> {
        let cursor = self.collection.find(filter, None).await;
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
        let cursor = self.collection.find_one(
            doc! { "_id": object_id }, 
            None 
        ).await;
        match cursor {
            Ok(c) => {
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

    pub async fn get_by_document(
        &mut self, 
        mut data: T, 
        obj:Document 
    ) -> Result<(), APIError> {
        let cursor = self.collection.find_one(
            obj.clone(),
            None 
        ).await;
        match cursor {
            Ok(c) => {
            if let Some(result) = c {
                data = result;
            } else {
                return Err(APIError::new(
                        StatusCode::NOT_FOUND,
                        format!(
                            "Could not find object with id: {}",
                            obj 
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

    pub async fn insert_one(
        &mut self,
        data: T,
    ) -> Result<(), APIError> {

        let cursor = self.collection
                    .insert_one(data, None).await;

        if let Err(e) = cursor {
            Err(APIError::new(
                    StatusCode::NOT_MODIFIED,
                    e.to_string()
            ))
        } else { Ok(()) }
    }

    pub async fn update_one(
        &mut self,
        filter: Document,
        updater: Document,
    ) -> Result<(), APIError> {
        let cursor = self.collection.update_one( 
            filter,
            updater,
            None
        ).await;

        match cursor {
            Ok(c) => {
                if c.modified_count == 0 {
                    Err(APIError::new(
                        StatusCode::NOT_MODIFIED,
                        "An error occured when modifying the database.".to_string()
                    ))
                } else { Ok(())}
            },
            Err(e) => {
                Err(
                    APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR, 
                        e.to_string()
                    ).clone())
            }
        }
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

