use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{Board, Post},
    DB,
};

#[get("/<board>/feed?<before>&<limit>")]
pub async fn feed(
    board: String,
    before: Option<u64>,
    limit: Option<u32>,
    mut db: Connection<DB>,
) -> Result<Json<Vec<Post>>, NotFound<Json<Value>>> {
    Board::get_feed(board, before, limit, &mut *db)
        .await
        .map(|p| Json(p))
}
