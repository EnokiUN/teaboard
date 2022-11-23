use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{models::Board, DB};

#[get("/")]
pub async fn index(mut db: Connection<DB>) -> Json<Vec<Board>> {
    Json(Board::all(&mut *db).await)
}
