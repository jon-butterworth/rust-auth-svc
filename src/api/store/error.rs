pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unable to create db pool")]
    UnableToCreateDbPool(String),
}
