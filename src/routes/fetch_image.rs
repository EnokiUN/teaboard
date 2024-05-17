use std::time::Duration;

use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{FetchResponse, Image},
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/images/<id>")]
pub async fn fetch_image<'a>(
    id: i64,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
) -> Response<Result<FetchResponse<'a>, NotFound<Json<Value>>>> {
    let mut ratelimiter = Ratelimiter::new("fetch-image", ip, 2, Duration::from_secs(5));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Image::get(id, &mut db).await)
}
