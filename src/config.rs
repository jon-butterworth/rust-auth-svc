use std::env;
use crate::{Error, Result};
use once_cell::sync::OnceCell;

pub fn config() -> &'static Config {
    static INSTANCE: OnceCell<Config> = OnceCell::new();

    INSTANCE.get_or_init(|| { Config::from_env().unwrap() } )
}

#[allow(non_snake_case)]
pub struct Config {
    pub PWD_KEY: Vec<u8>,
    pub DB_URL: String,
}

impl Config {
    fn from_env() -> Result<Config> {
        Ok(Config {
            PWD_KEY: get_env_b64_to_u8s("PWD_KEY")?,
            DB_URL: get_env("DATABASE_URL")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_b64_to_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
