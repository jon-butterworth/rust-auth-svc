use axum::{response::{IntoResponse, Response}, http::StatusCode};
use serde::Serialize;
use crate::api::store;
use crate::crypt;
use sqlx::error::DatabaseError;

#[derive(Debug, strum_macros::AsRefStr, thiserror::Error)]
pub enum Error {
    #[error("...")]
    SqlxError(#[from] sqlx::Error),
    #[error("...")]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("...")]
    UserAlreadyExists(String),
    #[error("...")]
    Base64DecodeError,
    #[error("...")]
    Crypt(crypt::error::Error),
    #[error("...")]
    Config(&'static str),
    #[error("...")]
    Store(#[from] store::Error),
    #[error("...")]
    ErrorUserDoesNotExist(String),
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
pub enum ClientError {
    UserAlreadyExists,
    InternalServerError,
    UserDoesNotExist,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESPONSE");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_error_map(&self) -> (StatusCode, ClientError) {
        println!("->> {:<12} - client_error_map", "INTO_RESPONSE");
        match self {
            Self::UserAlreadyExists(_) => {
                (StatusCode::BAD_REQUEST, ClientError::UserAlreadyExists)
            }
            Self::ErrorUserDoesNotExist(_) => {
                (StatusCode::BAD_REQUEST, ClientError::UserDoesNotExist)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR, ClientError::InternalServerError
            )
        }
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
    where
        E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|e| match e.into() {
            Error::SqlxError(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }
}
