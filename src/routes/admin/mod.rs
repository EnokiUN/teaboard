use std::time::Duration;

use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::{
    auth::PasswordAuth,
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache,
};

#[get("/admin/isadmin")]
pub async fn is_admin(
    mut cache: Connection<Cache>,
    ip: ClientIP,
    auth: PasswordAuth,
) -> Response<Json<PasswordAuth>> {
    let mut ratelimiter = Ratelimiter::new("isadmin", ip, 5, Duration::from_secs(10));
    ratelimiter.process_ratelimit(&mut cache).await?;
    ratelimiter.wrap_response(Json(auth))
}
