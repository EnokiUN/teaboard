use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{auth::StrictPasswordAuth, models::Post, DB};

#[post("/posts/<id>/pin")]
pub async fn pin(
    id: i64,
    mut db: Connection<DB>,
    _auth: StrictPasswordAuth,
) -> Result<(), NotFound<Json<Value>>> {
    Post::pin(id, &mut db).await
}
