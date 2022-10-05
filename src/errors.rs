use actix_web::{
    body::BoxBody, error::Error as ActixError, http::StatusCode, HttpResponse, ResponseError,
};
use std::{
    fmt::{self, Debug},
    io,
};
use tokio::sync::mpsc::error as mpsc_error;

use crate::DbAction;

#[derive(Debug)]
pub enum ApiError {
    Database(sea_orm::DbErr),
    Serialization(serde_json::Error),
    IoError(io::Error),
    Broadcast(mpsc_error::SendError<DbAction>),
    Actix(ActixError),
    NotFound(&'static str),
    OutdatedData,
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::IoError(err)
    }
}

impl From<mpsc_error::SendError<DbAction>> for ApiError {
    fn from(err: mpsc_error::SendError<DbAction>) -> ApiError {
        ApiError::Broadcast(err)
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        ApiError::Database(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::Serialization(err)
    }
}

impl From<ActixError> for ApiError {
    fn from(err: ActixError) -> Self {
        ApiError::Actix(err)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiError {:?}", self)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::OutdatedData => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error_details = match &self {
            ApiError::OutdatedData => {
                "Book from the request body has outdated updatedAt timestamp, send an actual one!"
                    .to_owned()
            }
            ApiError::NotFound(details) => format!("Not found: {}", details),
            _ => "Internal Server Error!".to_owned(),
        };
        HttpResponse::Ok().body(error_details)
    }
}
