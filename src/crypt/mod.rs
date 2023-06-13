pub mod error;

pub use self::error::{Error, Result};
use hmac::{Hmac, Mac};
use rand::RngCore;
use serde::Serialize;
use sha2::Sha512;
use crate::utils::{base64decode, base64encode};
use argon2::{Config, hash_encoded};
use axum::{Json, Router};
use axum::routing::get;
use tokio::task;
use uuid::Uuid;
use crate::config::config;

pub struct Password {
    pub content: String,
    pub salt: Option<String>,
}

impl Password {
    pub async fn encode(content: String, scheme: &str, salt: Option<String>) -> Result<(String, Option<String>)> {
        let salt = salt.unwrap_or_else(|| Uuid::new_v4().to_string());
        let password = Password { content, salt: Some(salt) };

        let key = &config().PWD_KEY;

        match scheme {
            "argon2" => password.argon().await.map(|enc| (enc, None)),
            "hmac" => password.hmac(key).map(|(enc, salt)| (enc, Some(salt))),
            _ => Err(Error::InvalidEncryptionScheme(
                String::from("Invalid Scheme. Choose argon2 or hmac"),
            )),
        }
    }

    async fn argon(&self) -> Result<String> {
        let argon_config = Config::default();
        let content = self.content.clone();
        let salt = self.salt.clone().ok_or(Error::MissingSalt)?;

        let pwd = task::spawn_blocking(move || {
            Ok(hash_encoded(content.as_bytes(), salt.as_bytes(), &argon_config)?)
        })
            .await
            .map_err(|_| Error::EncryptionError)?
            .map_err(|e| Error::ArgonEncError(e))?;

        Ok(base64encode(pwd.as_bytes()))
    }

    fn hmac(&self, key: &[u8]) -> Result<(String, String)> {
        let salt = self.salt.as_ref().ok_or(Error::MissingSalt)?;
        let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key)
            .map_err(|_| Error::SHA512ErrorEnc)?;
        hmac_sha512.update(self.content.as_bytes());
        hmac_sha512.update(salt.as_bytes());

        let res_enc = base64encode(hmac_sha512.finalize().into_bytes());
        let res_salt = base64encode(salt.to_string());

        Ok((res_enc, res_salt))
    }

    pub async fn verify_password(password: String, password_hash: String, hmac_salt: Option<String>) -> Result<()> {
        let decoded_pass = base64decode(&password_hash)
            .map_err(|_| Error::GenericError)?;

        let encoded;
        if decoded_pass.starts_with("$argon2i$") {
            encoded = match Password::encode(password, "argon2", None).await {
                Ok((enc, _)) => enc,
                Err(_) => return Err(Error::EncryptionError),
            };
        } else {
            let salt = hmac_salt.unwrap_or_else(|| Uuid::new_v4().to_string());
            encoded = match Password::encode(password, "hmac", Some(salt)).await {
                Ok((enc, _)) => enc,
                Err(_) => return Err(Error::EncryptionError),
            };
        }

        if encoded == password_hash {
            Ok(()) // <-- Add the token response? Or whatever may be required for auth.
        } else {
            Err(Error::PasswordNotMatching)
        }
    }
}
pub fn routes() -> Router {
    Router::new()
        .route("/api/crypt/new-key", get(gen_key))
}

#[derive(Serialize, Debug)]
pub struct EncKey {
    pub new_key: String,
}

pub async fn gen_key() -> Result<Json<String>> {
    let mut key = [0u8; 72];
    rand::thread_rng().fill_bytes(&mut key);
    let encoded = base64_url::encode(&key);

    let resp = EncKey {
        new_key: encoded
    };
    println!("Generated key: {:?}", resp);

    Ok(Json(resp.new_key))
}
