use std::error::Error;
use std::fmt::{Display, Result, Formatter, Debug};
use std::marker::PhantomData;

use axum::http::StatusCode;

#[derive(Clone)]
pub struct MKError<T: MKErrorKind + Sized> {
    pub message: String, // Changed to String
    pub kind: PhantomData<T>,
}

pub trait MKErrorKind {
    fn from_string(e: String) -> MKError<Self> where Self: Sized;
}

#[derive(Clone)]
pub struct InternalServiceError;
#[derive(Clone)]
pub struct RepositoryError;
#[derive(Clone)]
pub struct BadDataError;

impl MKErrorKind for InternalServiceError {
    fn from_string(e: String) -> MKError<Self> {
        MKError {
            message: e, // No need for &e, ownership is transferred
            kind: PhantomData,
        }
    }
}

impl MKErrorKind for RepositoryError {
    fn from_string(e: String) -> MKError<Self> {
        MKError {
            message: e, // No need for &e, ownership is transferred
            kind: PhantomData,
        }
    }
}

impl MKErrorKind for BadDataError {
    fn from_string(e: String) -> MKError<Self> {
        MKError {
            message: e, // No need for &e, ownership is transferred
            kind: PhantomData,
        }
    }
}

// Implement std::error::Error for MKError<InternalServiceError>
impl Error for MKError<InternalServiceError> {
    fn description(&self) -> &str {
        &self.message
    }
}

// Implement Display for MKError<InternalServiceError> for better formatting
impl Display for MKError<InternalServiceError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for MKError<InternalServiceError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MKError<InternalServiceError>: {}", self.message)
    }
}

impl From<Box<dyn Error>> for MKError<InternalServiceError> {
    fn from(value: Box<dyn Error>) -> Self {
        InternalServiceError::from_string(value.to_string())
    } 
}


impl Into<StatusCode> for MKError<InternalServiceError> {
    fn into(self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}


impl Error for MKError<BadDataError> {
    fn description(&self) -> &str {
        &self.message
    }
}

// Implement Display for MKError<BadDataError> for better formatting
impl Display for MKError<BadDataError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for MKError<BadDataError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MKError<BadDataError>: {}", self.message)
    }
}

impl From<Box<dyn Error>> for MKError<BadDataError> {
    fn from(value: Box<dyn Error>) -> Self {
        BadDataError::from_string(value.to_string())
    } 
}

impl Into<StatusCode> for MKError<BadDataError> {
    fn into(self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}


impl Error for MKError<RepositoryError> {
    fn description(&self) -> &str {
        &self.message
    }
}

// Implement Display for MKError<RepositoryError> for better formatting
impl Display for MKError<RepositoryError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

impl Debug for MKError<RepositoryError> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "MKError<RepositoryError>: {}", self.message)
    }
}

impl From<Box<dyn Error>> for MKError<RepositoryError> {
    fn from(value: Box<dyn Error>) -> Self {
        RepositoryError::from_string(value.to_string())
    } 
}

impl Into<StatusCode> for MKError<RepositoryError> {
    fn into(self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
}

impl From<surrealdb::Error> for MKError<RepositoryError> {
    fn from(value: surrealdb::Error) -> Self {
       RepositoryError::from_string(value.to_string()) 
    }
}

impl From<&str> for MKError<RepositoryError> {
    fn from(value: &str) -> Self {
        RepositoryError::from_string(value.to_string())
    }
}

impl From<&str> for MKError<InternalServiceError> {
    fn from(value: &str) -> Self {
        InternalServiceError::from_string(value.to_string())
    }
}

impl From<&str> for MKError<BadDataError> {
    fn from(value: &str) -> Self {
        BadDataError::from_string(value.to_string())
    }
}

pub type MKInternalServiceError = MKError<InternalServiceError>;
pub type MKRepositoryError = MKError<RepositoryError>;
pub type MKBadDataError = MKError<BadDataError>;
