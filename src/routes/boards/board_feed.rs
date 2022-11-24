use std::time::Duration;

use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{Board, PostInfo},
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/<board>/feed?<before>&<limit>")]
pub async fn feed(
    board: String,
    before: Option<u64>,
    limit: Option<u32>,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
) -> Response<Result<Json<Vec<PostInfo>>, NotFound<Json<Value>>>> {
    let mut ratelimiter =
        Ratelimiter::new(&format!("feed-{}", board), ip, 5, Duration::from_secs(10));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(
        Board::get_feed(board, before, limit, &mut *db)
            .await
            .map(|p| Json(p)),
    )
}
