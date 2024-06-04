use actix_web::{body::BoxBody, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InternalError {
    #[error("Invalid config value")]
    ParseConfig(String),

    #[error("SQL error")]
    Sql(#[from] sqlx::Error),
}

#[derive(Error, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,

    #[error("Internal Server Error")]
    Internal,
}

impl From<InternalError> for ApiError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::Sql(sqlx::Error::RowNotFound) => Self::NotFound,
            InternalError::Sql(_) => Self::Internal,
            InternalError::ParseConfig(_) => unreachable!(),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
