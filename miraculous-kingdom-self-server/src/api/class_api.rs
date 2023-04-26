pub mod class_routes {
    pub use super::get_class;
    pub use super::get_classes;
}

use crate::data_types::{
    characters::{Ability, Class, ClassEnum, ClassResponse},
    common::{
        verify_id,
        ClassDetailedResponse,
        DetailedResponse,
        Repository,
        VecClassDetailedResponse,
        MKModel 
    },
};
use axum::{extract::Path, Extension, Json};
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
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
pub async fn get_classes(Extension(mongo): Extension<Database>) 
-> Json<DetailedResponse<Vec<ClassResponse>>> {
    let mut response: DetailedResponse<Vec<Class>> = DetailedResponse::new(Vec::<Class>::new());
    let mut repository = Repository::<Class>::new(&mongo, "classes");
    response.run(|a| repository.get_all(a)).await;
    let mut resp = DetailedResponse::new(Vec::<ClassResponse>::new());
    resp.absorb(&mut response.clone());
    response.data.iter().for_each(|a| resp.data.push(a.as_response()));
    Json(resp)
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
        status = 400, 
        description = "Bad Request: id", 
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
pub async fn get_class(
    Extension(mongo): Extension<Database>,
    Path(id): Path<String>,
) -> Json<DetailedResponse<ClassResponse>> {
    let mut response: DetailedResponse<Class> = DetailedResponse::new(Class {
        class_id: ObjectId::new(),
        class_enum: ClassEnum::default(),
        class_desc: "".to_string(),
        class_perks: "".to_string(),
        class_abilities: Vec::<Ability>::new(),
        class_name: "dummy".to_string(),
    });

    let mut repository = Repository::<Class>::new(&mongo, "classes");

    if let Err(e) = verify_id(id, &mut response.data.class_id).await {
        response.clone().set_code(Some(e));
    }

    response
        .run(|a| repository.get_by_oid(a.clone(), a.data.class_id))
        .await;

    let mut resp = DetailedResponse::new(response.data.as_response());
    Json(resp.absorb(&mut response.clone()))
}
