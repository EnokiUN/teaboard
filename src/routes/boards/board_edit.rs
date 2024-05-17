use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{auth::StrictPasswordAuth, models::Board, DB};

#[patch("/boards/<id>?<description>")]
pub async fn edit(
    id: &str,
    description: Option<String>,
    mut db: Connection<DB>,
    _auth: StrictPasswordAuth,
) -> Result<(), NotFound<Json<Value>>> {
    Board::edit(id, description, &mut db).await
}
