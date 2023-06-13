use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("...")]
    GenericError,
    #[error("Unable to encode")]
    SHA512ErrorEnc,
    #[error("Crypt API Error")]
    CryptAPIHandlerError,
    #[error("Unrecognized encryption scheme")]
    InvalidEncryptionScheme(String),
    #[error("Encryption scheme not found")]
    SchemeNotFoundInContent,
    #[error("Incorrect password")]
    PasswordNotMatching,
    #[error("Argon2 encryption error")]
    ArgonEncError(#[from] argon2::Error),
    #[error("Encryption error")]
    EncryptionError,
    #[error("Missing salt")]
    MissingSalt,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}