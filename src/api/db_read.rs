use axum::Json;
use axum::extract::State;
use serde_json::{json, Value};
use crate::api::AppState;
use crate::api::Error;

pub async fn db_read (State(state): State<AppState>) -> Result<Json<Value>, Error> {
    let user = sqlx::query!(
        "SELECT * FROM users WHERE username = $1",
        "jonbut34".to_string()
        )
        .fetch_all(state.db())
        .await?;

    let json_result = user.into_iter()
        .map(|user| json!({
            "id": user.id,
            "first_name": user.first_name,
            "surname": user.surname,
            "email": user.email,
        }))
        .collect();
    Ok(Json(json_result))
}