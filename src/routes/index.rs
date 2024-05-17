use std::time::Duration;

use rocket::{serde::json::Json, State};
use rocket_db_pools::Connection;

use crate::{
    conf::Conf,
    models::{Board, InstanceInfo},
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[get("/")]
pub async fn index(
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
    conf: &State<Conf>,
) -> Response<Json<InstanceInfo>> {
    let mut ratelimiter = Ratelimiter::new("info", ip, 20, Duration::from_secs(10));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Json(InstanceInfo {
        info: conf.inner().clone(),
        boards: Board::all(&mut db).await,
    }))
}
