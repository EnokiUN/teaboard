use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{auth::StrictPasswordAuth, models::Board, DB};

#[delete("/boards/<id>")]
pub async fn delete<'a>(
    id: &'a str,
    mut db: Connection<DB>,
    _auth: StrictPasswordAuth,
) -> Result<(), NotFound<Json<Value>>> {
    Board::delete(id, &mut *db).await
}
