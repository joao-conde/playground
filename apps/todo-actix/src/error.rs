use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum InternalError {
    Sql(sqlx::Error),
}

impl From<sqlx::Error> for InternalError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sql(err)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ApiError {
    NotFound,
    Internal,
}

impl From<InternalError> for ApiError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::Sql(sqlx::Error::RowNotFound) => Self::NotFound,
            _ => Self::Internal,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "Not Found"),
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
