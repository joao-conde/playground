use actix_web::{body::BoxBody, error::BlockingError, http::StatusCode, HttpResponse};
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Internal(String),
}

#[derive(Debug)]
pub enum InternalError {
    NotFound,
    Internal(String),
}

impl From<InternalError> for ApiError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::NotFound => Self::NotFound,
            InternalError::Internal(msg) => Self::Internal(msg),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not Found Error"),
            ApiError::Internal(msg) => write!(f, "Internal Server Error: {msg}"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<rusqlite::Error> for InternalError {
    fn from(err: rusqlite::Error) -> Self {
        match err {
            rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
            _ => Self::Internal(err.to_string()),
        }
    }
}

impl From<BlockingError> for InternalError {
    fn from(err: BlockingError) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<r2d2::Error> for InternalError {
    fn from(err: r2d2::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
