use actix_web::{body::BoxBody, error::BlockingError, http::StatusCode, HttpResponse};
use std::fmt::Display;

#[derive(Debug)]
pub enum InternalError {
    Sql(rusqlite::Error),
    ConnectionPool(r2d2::Error),
    Actix(actix_web::Error),
}

impl From<rusqlite::Error> for InternalError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sql(err)
    }
}

impl From<BlockingError> for InternalError {
    fn from(err: BlockingError) -> Self {
        Self::Actix(err.into())
    }
}

impl From<r2d2::Error> for InternalError {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionPool(err)
    }
}

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Internal,
}

impl From<InternalError> for ApiError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::Sql(rusqlite::Error::QueryReturnedNoRows) => Self::NotFound,
            _ => Self::Internal,
        }
    }
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
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
