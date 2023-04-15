use super::common::{APIError, DetailedResponse};
use axum::http::StatusCode;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection, Database,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Repository<
    T: Default + Clone + Serialize + Sync + Send + Unpin + DeserializeOwned + std::fmt::Debug,
> {
    pub model: T,
    pub collection: Collection<T>,
    phantom: PhantomData<T>,
}

impl<T: Default + Clone + Serialize + Sync + Send + Unpin + DeserializeOwned + std::fmt::Debug>
    Repository<T>
{
    pub fn new(db: &Database, coll_name: &str) -> Self {
        Repository {
            model: T::default(),
            collection: db.clone().collection::<T>(coll_name),
            phantom: PhantomData,
        }
    }

    pub async fn get_all(
        &mut self,
        mut data: DetailedResponse<Vec<T>>,
    ) -> DetailedResponse<Vec<T>> {
        let cursor = self.collection.find(None, None).await;
        match cursor {
            Ok(mut c) => {
                while let Ok(res) = c.try_next().await {
                    match res {
                        Some(r) => data.data.push(r),
                        None => {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                return data
                    .set_code(Some(APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )))
                    .clone();
            }
        }
        data
    }

    pub async fn get_all_with_filter(
        &mut self,
        mut data: DetailedResponse<Vec<T>>,
        filter: Document,
    ) -> DetailedResponse<Vec<T>> {
        let cursor = self.collection.find(filter, None).await;
        match cursor {
            Ok(mut c) => {
                while let Ok(res) = c.try_next().await {
                    match res {
                        Some(r) => data.data.push(r),
                        None => {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                return data
                    .set_code(Some(APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )))
                    .clone();
            }
        }
        data
    }

    pub async fn get_by_oid(
        &mut self,
        mut data: DetailedResponse<T>,
        object_id: ObjectId,
    ) -> DetailedResponse<T> {
        let cursor = self
            .collection
            .find_one(doc! { "_id": object_id }, None)
            .await;
        match cursor {
            Ok(c) => {
                if let Some(result) = c {
                    data.data = result;
                } else {
                    return data
                        .set_code(Some(APIError::new(
                            StatusCode::NOT_FOUND,
                            format!("Could not find object with id: {}", object_id.to_hex()),
                        )))
                        .clone();
                }
            }
            Err(e) => {
                return data
                    .set_code(Some(APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )))
                    .clone();
            }
        }
        data
    }

    pub async fn get_by_document(
        &mut self,
        mut data: DetailedResponse<T>,
        obj: Document,
    ) -> DetailedResponse<T> {
        let cursor = self.collection.find_one(obj.clone(), None).await;
        match cursor {
            Ok(c) => {
                if let Some(result) = c {
                    data.data = result;
                } else {
                    return data
                        .set_code(Some(APIError::new(
                            StatusCode::NOT_FOUND,
                            format!("Could not could find with criteria: {}", obj),
                        )))
                        .clone();
                }
            }
            Err(e) => {
                return data
                    .set_code(Some(APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )))
                    .clone();
            }
        }
        data
    }

    pub async fn insert_one(
        &mut self,
        object: T,
        mut data: DetailedResponse<T>,
    ) -> DetailedResponse<T> {
        let cursor = self.collection.insert_one(object.clone(), None).await;

        if let Err(e) = cursor {
            return data
                .set_code(Some(APIError::new(StatusCode::NOT_MODIFIED, e.to_string())))
                .clone();
        } else {
            data.data = object;
        }
        data
    }

    pub async fn update_one(
        &mut self,
        filter: Document,
        updater: Document,
        mut data: DetailedResponse<T>,
    ) -> DetailedResponse<T> {
        let cursor = self.collection.update_one(filter, updater, None).await;

        match cursor {
            Ok(c) => {
                if c.modified_count == 0 {
                    return data
                        .set_code(Some(APIError::new(
                            StatusCode::NOT_MODIFIED,
                            "An error occured when modifying the database.".to_string(),
                        )))
                        .clone();
                } else {
                    data
                }
            }
            Err(e) => {
                return data
                    .set_code(Some(APIError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        e.to_string(),
                    )))
                    .clone();
            }
        }
    }
}

pub async fn verify_id(id: String, object_id: &mut ObjectId) -> Result<(), APIError> {
    if let Ok(oid) = ObjectId::parse_str(&id) {
        *object_id = oid.to_owned();
        Ok(())
    } else {
        Err(APIError::new(
            StatusCode::BAD_REQUEST,
            format!("{} is not a valid objectId.", &id),
        ))
    }
}
