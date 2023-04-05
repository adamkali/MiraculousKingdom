pub mod class_routes {
    pub use super::get_all;
    pub use super::get;
}

use axum::{
    Extension,
    http::StatusCode,
    Json,
    extract::Path,
};
use mongodb::{
    Database,
    bson::{
        oid::ObjectId,
        doc
    },
};
use crate::data_types::{
    characters::{
        Class,
        Ability
    },
    common::{
        DetailedResponse,
        VecClassDetailedResponse,
        ClassDetailedResponse
    }
};
use futures::stream::TryStreamExt;


#[utoipa::path(
    get,
    path = "/api/class",
    responses((
        status = 200, 
        description = "Listed classes from database", 
        body = VecClassDetailedResponse 
    ),
    (
        status = 500, 
        description = "Internal error occured", 
        body = VecClassDetailedResponse 
    ))
)]
pub async fn get_all(
    Extension(mongo): Extension<Database>
) -> Json<DetailedResponse<Vec<Class>>> {
    let mut response: DetailedResponse<Vec<Class>> =
        DetailedResponse::new(Vec::<Class>::new());

    let collection = mongo.clone().collection::<Class>("classes");
    let cursor = collection.find(None, None).await;

    match cursor {
        Ok(mut c) => {
            while let Ok(res) = c.try_next().await {
                match res {
                    Some(r) => {
                        response.data.push(r)
                    },
                    None => {
                        response.set_code(StatusCode::OK, "".to_string());
                        break;
                    },
                }
            };
        },
        Err(e) => {
            response.set_code(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    }
    Json(response)
}


#[utoipa::path(
    get,
    path = "/api/class/{id}",
    responses((
        status = 200, 
        description = "Found class from database", 
        body = ClassDetailedResponse
    ),
    (
        status = 500, 
        description = " Internal error occured", 
        body = ClassDetailedResponse
    )),
    params(
        ("id" = String, Path, description = "ObjectId for mongodb")
    )
)]
pub async fn get(
    Extension(mongo): Extension<Database>,
    Path(id): Path<String>
) -> Json<DetailedResponse<Class>> {
    let mut response: DetailedResponse<Class> =
        DetailedResponse::new(Class { 
            class_id: ObjectId::new(), 
            class_desc: "".to_string(), 
            class_perks: "".to_string(), 
            class_abilities: Vec::<Ability>::new(), 
            class_name: "dummy".to_string()
        }); 

    let mut object_id: ObjectId = ObjectId::new();
    match ObjectId::parse_str(&id) {
        Ok(oid) => {
            object_id = oid;
        },
        Err(e) => {
            return Json(response.set_code(StatusCode::BAD_REQUEST, e.to_string()).clone());
        }
    };


    let collection = mongo.clone().collection::<Class>("classes");
    let cursor = collection.find_one(doc! { "_id": object_id }, None ).await;

    match cursor {
        Ok(c) => {
            if let Some(result) = c {
                response.data = result;
                response.set_code(StatusCode::OK, "".to_string());
            } else {
                response.set_code(StatusCode::NOT_FOUND, 
                                  format!("Could not find class with id {}", id));
            }
        },
        Err(e) => {
            response.set_code(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
        }
    }  
    Json(response)
}
