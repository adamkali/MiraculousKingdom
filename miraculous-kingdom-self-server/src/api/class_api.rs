pub mod class_routes {
    pub use super::get_all;
    pub use super::get;
}

use axum::{
    Extension,
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
        ClassDetailedResponse,
        Repository,
        verify_id,
    }
};

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

    let mut repository = Repository::<Class>::new(&mongo, "classes");

    Json(
        response.run(|a| 
            repository.get_all(a)
        ).await
    )
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

    let mut repository = Repository::<Class>::new(&mongo, "classes");

    verify_id(id, &mut response.data.class_id);
    
    Json(response
         .run(|a| repository.get_by_oid(a.clone(), a.class_id))
         .await
    )
}
