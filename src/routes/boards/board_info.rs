use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{models::Board, DB};

#[get("/<board>")]
pub async fn info(
    board: &str,
    mut db: Connection<DB>,
) -> Result<Json<Board>, NotFound<Json<Value>>> {
    Board::get(board, &mut *db).await.map(|p| Json(p))
}
