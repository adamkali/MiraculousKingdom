use serde_json;
use serde::Serialize;
use axum::{
    http::{
        StatusCode,
        HeaderMap,
    },
    body::{
        HttpBody,
        Bytes,
    },
    response::IntoResponse,
};
use std::{
    convert::Infallible,
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};

use crate::data_types::characters::{
    Class,
    Character
};
use crate::data_types::engine::GameInfo;

#[derive(Serialize, Clone, utoipa::ToSchema)]
#[aliases(
    VecClassDetailedResponse = DetailedResponse<Vec<Class>>,
    ClassDetailedResponse = DetailedResponse<Class>,
    GameInfoDetailedResponse = DetailedResponse<GameInfo>,
    GamesInfoDetailedResponse = DetailedResponse<Vec<GameInfo>>,
    PassDetailedResponse = DetailedResponse<String>,
    CharAddedDetailedResponse = DetailedResponse<Character>,
    VecCharDetailedResponse = DetailedResponse<Vec<Character>>,
    CharDetialedResponse = DetailedResponse<Character>,
)]
pub struct DetailedResponse<T: Serialize + Clone> {
    pub data: T,
    pub success: bool,
    pub message: String,
    pub code: u16,
}

impl<T: Serialize + Send + Clone> DetailedResponse<T> {
    pub fn new(d: T) -> DetailedResponse<T> {
        DetailedResponse { 
            data: d, 
            success: true, 
            message: "".to_string(), 
            code: StatusCode::OK.as_u16()
        }
    }

    pub fn set_code(&mut self, code: StatusCode, message: String) -> &mut Self {
        self.code = code.as_u16();
        self.success = code.is_success();
        self.message = message;
        self
    }

    pub fn nil(message: String) -> String {
        let nil: DetailedResponse<Option<bool>> =
            DetailedResponse {
                data: None,
                success: false,
                message,
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            };
        serde_json::to_string(&nil)
            .unwrap_or_else(|_| "well... $hit".to_string())
    }

    pub fn run<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Self) -> Result<(), APIError>
    {
        if self.success {
            match f(self) {
                Ok(_) => {self.set_code(StatusCode::OK, "Ok".to_string());},
                Err(e) => {self.set_code(e.status_code, e.message);},
            }
        }
        self
    }
}

impl<T: Serialize + Send + Clone> HttpBody for DetailedResponse<T> {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_data(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        // Serialize the DetailedResponse to a json String:
        let json_str = match serde_json::to_string(&*self) {
            Ok(s) => s,
            Err(e) => DetailedResponse::<Option<bool>>::nil(e.to_string())
        };

        let data = Bytes::from(json_str);
        Poll::Ready(Some(Ok(data)))
    }
    
    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

impl<T: Serialize + Send + Clone + 'static> IntoResponse for DetailedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::new(axum::body::boxed(self))
    }
}

#[derive(Clone)]
pub struct APIError {
    pub status_code: StatusCode,
    pub message: String,
}

impl APIError {
    pub fn new(status_code: StatusCode, message: String) -> APIError {
        APIError { status_code, message }
    }
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status Code: {} occured. Error: {}",
            &self.status_code.as_u16(),
            &self.message
        )
    }
}

impl std::fmt::Debug for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status Code: {} occured. Error: {}",
            &self.status_code.as_u16(),
            &self.message
        )
    }
}
