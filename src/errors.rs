use actix_web::{body::BoxBody, http, HttpResponse, ResponseError};
use std::{fmt, io};

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(sea_orm::DbErr),
    SerializationError(serde_json::Error),
    IoError(io::Error),
    NotFoundError,
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::IoError(err)
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        ApiError::DatabaseError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::SerializationError(err)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiError {:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::Ok().body("Internal server error!")
    }
}
