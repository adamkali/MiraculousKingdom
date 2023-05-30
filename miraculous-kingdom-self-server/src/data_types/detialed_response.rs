use axum::{
    body::{Bytes, HttpBody},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    async_trait,
};
use futures::Future;
use serde::Serialize;
use serde_json;
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    pin::Pin,
    task::{Context, Poll},
};

use super::common::{MKModel, MkResponse};
use crate::data_types::characters::{CharacterResponse, ClassResponse};
use crate::data_types::engine::{GameInfo, SeasonResponse};
use crate::data_types::queue::QueueResonse;

#[derive(Serialize, Clone, utoipa::ToSchema)]
pub enum Progress {
    Succeeding,
    Failing(APIError),
}

#[derive(Serialize, Clone, utoipa::ToSchema)]
#[aliases(
    VecClassDetailedResponse = DetailedResponse<Vec<ClassResponse>>,
    ClassDetailedResponse = DetailedResponse<ClassResponse>,
    GameInfoDetailedResponse = DetailedResponse<GameInfo>,
    GamesInfoDetailedResponse = DetailedResponse<Vec<GameInfo>>,
    PassDetailedResponse = DetailedResponse<String>,
    CharAddedDetailedResponse = DetailedResponse<CharacterResponse>,
    VecCharDetailedResponse = DetailedResponse<Vec<CharacterResponse>>,
    CharDetialedResponse = DetailedResponse<CharacterResponse>,
    SeasonDetailedResponse = DetailedResponse<SeasonResponse>,
    SeasonsDetailedResponse = DetailedResponse<Vec<SeasonResponse>>,
    QueueDetailedResponse = DetailedResponse<QueueResonse>,
)]
pub struct DetailedResponse<T: Clone + Serialize> {
    pub data: T,
    pub success: Progress,
}

impl<T: Serialize + Send + Clone> DetailedResponse<T> {
    pub fn new(d: T) -> DetailedResponse<T> {
        DetailedResponse {
            data: d,
            success: Progress::Succeeding,
        }
    }

    pub fn set_code(&mut self, error: Option<APIError>) -> &mut Self {
        if let Some(err) = error {
            self.success = Progress::Failing(err.clone());
        } else {
            self.success = Progress::Succeeding;
        }
        self
    }

    pub fn nil(message: String) -> String {
        let nil: DetailedResponse<Option<bool>> = DetailedResponse {
            data: None,
            success: Progress::Failing(APIError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                message.clone(),
            )),
        };
        serde_json::to_string(&nil).unwrap_or_else(|_| "well... $hit".to_string())
    }

    pub async fn run<F, Fut>(&mut self, f: F) -> Self
    where
        F: FnOnce(Self) -> Fut,
        Fut: Future<Output = Self>,
    {
        if let Progress::Succeeding = self.success {
            let res = f(self.clone()).await;
            if let Progress::Succeeding = res.success {
                *self = res;
            } else if let Progress::Failing(e) = res.success {
                self.set_code(Some(e));
            }
        }
        self.clone()
    }

    pub fn absorb<S>(&mut self, to: &mut DetailedResponse<S>) -> Self
    where
        S: Serialize + Send + Clone,
    {
        to.success = self.success.clone();
        self.clone()
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
        let json_str = match serde_json::to_string(&self.clone()) {
            Ok(s) => s,
            Err(_e) => "An error occured while serializing the result...".to_string(),
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

impl<T> Display for DetailedResponse<T>
where
    T: Serialize + Send + Clone + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", serde_json::to_string(&self.clone()))
    }
}

impl<T> Debug for DetailedResponse<T>
where
    T: Serialize + Send + Clone + Sized + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", serde_json::to_string_pretty(&self.clone()))
    }
}

#[derive(Serialize, Clone, utoipa::ToSchema)]
pub struct APIError {
    pub status_code: u16,
    pub message: String,
}

impl APIError {
    pub fn new(status_code: StatusCode, message: String) -> APIError {
        APIError {
            status_code: status_code.as_u16(),
            message,
        }
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status Code: {} occured. Error: {}",
            &self.status_code, &self.message
        )
    }
}

impl Debug for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status Code: {} occured. Error: {}",
            &self.status_code, &self.message
        )
    }
}

