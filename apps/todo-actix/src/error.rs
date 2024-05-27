use actix_web::{body::BoxBody, error::BlockingError, http::StatusCode, HttpResponse};
use std::fmt::Display;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Internal,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not Found Error"),
            ApiError::Internal => write!(f, "Internal Server Error"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::new(self.status_code())
    }
}

impl From<rusqlite::Error> for ApiError {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
            _ => Self::Internal,
        }
    }
}

impl From<BlockingError> for ApiError {
    fn from(_: BlockingError) -> Self {
        Self::Internal
    }
}

impl From<r2d2::Error> for ApiError {
    fn from(_: r2d2::Error) -> Self {
        Self::Internal
    }
}
