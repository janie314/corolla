use axum::{http::StatusCode, response::IntoResponse};

/// An error type for a SQLite DB. Wraps several types of errors and implements axum_core::response::into_response::IntoResponse.
#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    JSON(serde_json::Error),
    Server,
    SQL(sqlx::Error),
    QueryDoesNotExist,
    WrongNumberOfArgs,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::File(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JSON(e)
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Error::SQL(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "there was a problem running your query",
        )
            .into_response()
    }
}
