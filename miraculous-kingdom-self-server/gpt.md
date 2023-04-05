Hi chat gpt, I would like you to help me improve my DetailedResponse structure that is written in rust. It is a communication layer for my Axum web app. Consider the following code:
```
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


#[derive(Serialize, Clone)]
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
            success: false, 
            message: "".to_string(), 
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16()}
    }

    pub fn set_code(&mut self, code: StatusCode) -> Self {
        self.code = code.as_u16();
        self.success = code.is_success();
        (*self).clone()
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
}

impl<T: Serialize + Send + Clone> HttpBody for DetailedResponse<T> {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_data(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        // Serialize the DetailiedResponse to a json String:
        let json_str = match serde_json::to_string(&*self) {
            Ok(s) => s,
            Err(e) => DetailedResponse::<Option::<bool>>::nil(e.to_string())
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

impl<T: Serialize + Send + Clone +  'static> IntoResponse for DetailedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::new(axum::body::boxed(self))
    }
}
```
Above my current code. Can you help me make it into a monad?
Here is what I mean. 
there should be a function associated to the Detailed response.
```
    pub fn run() -> DetailedResponse<T>
```
where the run function should do some things,
- its input should be a function called like this:
```
response.run(|a| a.get_all_from_db(mongo).await?)
        .run(|a| a.update_price_on_db(mongo, 500).await?)
```
- any function called insidde of the run function should terminate into a 
```
Result<(), APIError<StatusCode, String>>
```
- then internally it checks to see if there was an APIError or not.
```
    match (result of the passed in function) {
        Ok(data potentially stored in matched result: a) => {
            &self.set_code(StatusCode::OK);
            // do something whith a!
        },
        Err(e: StatusCode, m: String) => {
            &self.set_code(e);
            &self.message = m;
        }
    }
```
- then the run function locks itself down if success is false. it will not do anything else in the result.
```
if *self.success {
    match( result of the passed in function) {
        // branches here
    }
}
// if not don not do anything so that no operatios occur during the rest of the api call.
```
That way the detailed response struct becomes so much stronger.
