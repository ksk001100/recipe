use axum::{
    body::{Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::convert::Infallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ServiceError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("JWKSFetchError: {0}")]
    JWKSFetchError(String),
}

impl IntoResponse for ServiceError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status, error_message) = match self {
            ServiceError::InternalServerError(msg) | ServiceError::JWKSFetchError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

// #[derive(thiserror::Error, Debug, Clone)]
// pub enum Error {
//     #[error("Bad config: {0}")]
//     BadConfig(String),
//     #[error("Connecting to database: {0}")]
//     ConnectingToDatabase(String),
//     #[error("Internal error: {0}")]
//     Internal(String),
//     #[error("Not found: {0}")]
//     NotFound(String),
//     #[error("Migrating database: {0}")]
//     DatabaseMigration(String),
//     #[error("Invalid argument: {0}")]
//     InvalidArgument(String),
// }
//
// impl IntoResponse for Error {
//     type Body = Full<Bytes>;
//     type BodyError = Infallible;
//
//     fn into_response(self) -> Response<Self::Body> {
//         let (status, error_message) = match self {
//             Error::BadConfig(message)
//             | Error::ConnectingToDatabase(message)
//             | Error::Internal(message)
//             | Error::DatabaseMigration(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
//             Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
//             Error::InvalidArgument(message) => (StatusCode::BAD_REQUEST, message),
//         };
//
//         let body = Json(json!({
//             "error": error_message,
//         }));
//
//         (status, body).into_response()
//     }
// }
