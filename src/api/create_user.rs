use axum::Json;
use axum::extract::State;
use crate::api::{AppState, UserResp, UserFull};
use crate::api::Error;
use crate::api::error::ResultExt;
use crate::crypt::Error::EncryptionError;
use crate::crypt::Password;

pub async fn add_user(State(state): State<AppState>, body: Json<UserFull>) -> Result<Json<UserResp>, Error> {
    let full_name = format!("{} {}", body.first_name, body.surname);
    println!("->> {:<12} - add_user - {}", "API_HANDLER", full_name);

    let hash_password = match Password::encode(body.password.to_string(), "argon2", None).await {
        Ok((enc, _)) => enc,
        Err(_) => return Err(Error::Crypt(EncryptionError)),
    };

    let user_id = sqlx::query_scalar!(
        "INSERT INTO users (first_name, surname, email, username, password) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        body.first_name.to_string(), body.surname.to_string(), body.email.to_string(), body.username.to_string(),
        hash_password
        )
        .fetch_one(state.db())
        .await
        .on_constraint("email", |_| {
            Error::UserAlreadyExists(body.email.to_string())
        })?;

    Ok(Json(UserResp{
        id: user_id,
        name: full_name.to_string(),
        email: body.email.to_string(),
    }))
}