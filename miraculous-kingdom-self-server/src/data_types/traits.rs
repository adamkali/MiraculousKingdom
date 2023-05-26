pub trait MKModel {
    type Response: Sized + MkResponse;
    fn as_response(&self) -> Self::Response;
}

pub trait MkResponse {

}
