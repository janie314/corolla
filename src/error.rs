use axum::{http::StatusCode, response::IntoResponse};

/// An error type for a SQLite DB. Wraps several types of errors and implements axum_core::response::into_response::IntoResponse.
#[derive(Debug)]
pub enum Error {
    SQL(sqlx::Error),
    QueryDoesNotExist,
    WrongNumberOfArgs,
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::SQL(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "what was the problem").into_response()
    }
}
