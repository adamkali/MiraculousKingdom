use loco_rs::prelude::format::json;
use loco_rs::prelude::Response;
use loco_rs::prelude::Result;
use serde::Serialize;
use utoipa::ToSchema;

use crate::controllers::classes::ClassResponse;


#[derive(Debug, Serialize, ToSchema)]
#[aliases(
    ClassListDetailedResponse = DetailedResponse<Vec<ClassResponse>>,
)]
pub struct DetailedResponse<DataType>
where
    DataType: Serialize,
{
    data: Option<DataType>,
    successful: bool,
    message: Option<String>,
    next_link: Option<String>,
}


impl<DataType> DetailedResponse<DataType> 
where DataType: Serialize {
    pub fn new(data: Option<DataType>)
        -> DetailedResponse<DataType> {
        DetailedResponse::<DataType> {
            data,
            successful: false,
            message: None,
            next_link: None,
        }
    }

    pub fn fail(
        data: Option<DataType>,
        status_code: u16,
        next_link: Option<String>,
        err: impl std::error::Error,
    ) -> DetailedResponse<DataType> {
        let mut response =  DetailedResponse::new(data);
        response.message= Some(format!("{}: {}", status_code, err));
        response.next_link = next_link;
        response
    }

    pub fn ok(
        data: Option<DataType>,
        next_link: Option<String>
    ) -> DetailedResponse<DataType> {
        let mut response = DetailedResponse::new(data);
        response.message = Some("OK".to_string());
        response.successful = true;
        response.next_link = next_link;
        response
    }

    pub fn json(&self) -> Result<Response> { json(self) }
}
