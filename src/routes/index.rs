use std::time::Duration;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{
    models::Board,
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/")]
pub async fn index(
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
) -> Response<Json<Vec<Board>>> {
    let mut ratelimiter = Ratelimiter::new("info", ip, 2, Duration::from_secs(10));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Json(Board::all(&mut *db).await))
}
