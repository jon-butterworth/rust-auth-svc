use crate::{api, crypt};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, strum_macros::AsRefStr, thiserror::Error)]
pub enum Error {
    #[error("...")]
    GenericError,
    #[error("...")]
    Base64DecodeError,
    #[error("...")]
    Crypt(crypt::error::Error),
    #[error("...")]
    Api(#[from] api::Error),
    #[error("...")]
    ConfigMissingEnv(&'static str),
    #[error("...")]
    ConfigWrongFormat(&'static str),

}