use std::time::Duration;

use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{Post, PostInfo},
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/posts/<id>")]
pub async fn get(
    id: u64,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
) -> Response<Result<Json<PostInfo>, NotFound<Json<Value>>>> {
    let mut ratelimiter = Ratelimiter::new("fetch-post", ip, 10, Duration::from_secs(5));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Post::info(id, &mut db).await.map(Json))
}
