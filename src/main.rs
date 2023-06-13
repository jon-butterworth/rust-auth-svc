mod error;
mod api;
mod utils;
mod crypt;
mod config;

pub use self::error::{Error, Result};
use api::AppState;

#[tokio::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();
    let state = AppState::new().await?;
    api::serve(state).await?;

    Ok(())


}