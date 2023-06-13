use axum::Json;
use axum::extract::{Path, State};
use serde_json::{json, Value};
use crate::api::{AppState, Error};

pub async fn db_delete(State(state): State<AppState>, Path(user): Path<String>) -> Result<Json<Value>, Error> {
    println!("->> {:<12} - delete_user - {}", "API_HANDLER", user);
    let user_del = sqlx::query("DELETE FROM users WHERE email = $1")
        .bind(&user)
        .execute(state.db())
        .await?
        .rows_affected();

    if user_del == 0 {
        return Err(Error::ErrorUserDoesNotExist(user))
    }

    let resp = json!({
        "message": "User deleted",
        "email": user.to_string()
    });

    Ok(Json(resp))
}