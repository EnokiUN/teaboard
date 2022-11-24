use std::time::Duration;

use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::Board,
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/<board>")]
pub async fn info(
    board: &str,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
) -> Response<Result<Json<Board>, NotFound<Json<Value>>>> {
    let mut ratelimiter =
        Ratelimiter::new(&format!("info-{}", board), ip, 5, Duration::from_secs(10));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Board::get(board, &mut *db).await.map(|p| Json(p)))
}
