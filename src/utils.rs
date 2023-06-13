
//use crate::{Error, Result};
use crate::Error;
use crate::Result;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn base64encode<T: AsRef<[u8]>>(content: T) -> String {
    base64_url::encode(content.as_ref())
}

pub fn base64decode(b64u: &String) -> Result<String> {
    let decoded = base64_url::decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::Base64DecodeError)?;
    Ok(decoded)
}
