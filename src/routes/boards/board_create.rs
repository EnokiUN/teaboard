use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{auth::StrictPasswordAuth, models::Board, DB};

#[post("/boards", data = "<board>")]
pub async fn create(
    board: Json<Board>,
    mut db: Connection<DB>,
    _auth: StrictPasswordAuth,
) -> Json<Board> {
    Json(Board::create(board.into_inner(), &mut db).await)
}
