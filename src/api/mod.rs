mod get_user;
mod create_user;
pub(crate) mod error;
mod store;
mod login;
mod delete_user;

use std::net::SocketAddr;
use axum::{Json, middleware, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ulid::Ulid;
use crate::api::store::{new_db_pool, Db};
pub use error::Error;
use crate::api::get_user::db_read;
use crate::api::create_user::add_user;
use crate::api::delete_user::db_delete;
use crate::api::store::Error::UnableToCreateDbPool;
use crate::crypt;


#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UserFull {
    pub first_name: String,
    pub surname: String,
    pub email: String,
    pub username:  String,
    pub password: String,
}

// #[derive(Deserialize, Debug)]
// pub struct UserLogin {
//     pub username: String,
//     pub password: String,
// }

#[derive(Serialize, Deserialize)]
pub struct UserResp {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        let db = new_db_pool().await.map_err(|e| Error::Store(UnableToCreateDbPool(e.to_string())))?;

        Ok::<Self, Error>(Self { db })
    }
    pub(in crate::api) fn db(&self) -> &Db {
        &self.db
    }
}

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/api/user/add", post(add_user))
        .route("/api/db/read", get(db_read))
        .route("/api/user/delete/:user", delete(db_delete))
        .with_state(state)
}

async fn client_response_mapper(res: Response) -> Response {
    println!("->> {:12} - client_response_mapper", "RESP_MAPPER");
    let ulid = Ulid::new();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se|se.client_error_map());

    let err_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "ulid": ulid.to_string(),
                }
            });
        (*status_code, Json(client_error_body)).into_response()

        });
    println!("--> server log line - {ulid} - Error: {service_error:?}");
    err_response.unwrap_or(res)
}

pub async fn serve(state: AppState) -> Result<(), Error> {

    let api = Router::new()
        .merge(routes(state.clone()))
        .merge(crypt::routes())
        .layer(middleware::map_response(client_response_mapper));

    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");

    axum::Server::bind(&addr)
        .serve(api.into_make_service())
        .await
        .expect("Error running API routes");


    Ok::<(), Error>(())
}
