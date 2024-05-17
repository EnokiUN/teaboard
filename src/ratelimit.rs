use std::{
    convert::Infallible,
    fmt::Display,
    net::IpAddr,
    str::FromStr,
    time::{Duration, SystemTime},
};

use rocket::{
    http::Header,
    request::{FromRequest, Outcome, Request},
    response::Responder,
    serde::json::Json,
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde_json::{json, Value};

use crate::Cache;

pub type Response<T> = Result<RatelimitWrapper<T>, RatelimitWrapper<TooManyRequests>>;

#[derive(Debug, Responder)]
pub struct RatelimitWrapper<T> {
    pub inner: T,
    pub ratelimit_reset: Header<'static>,
    pub ratelimit_max: Header<'static>,
    pub ratelimit_last_reset: Header<'static>,
    pub ratelimit_request_count: Header<'static>,
}

#[derive(Debug, Responder)]
#[response(content_type = "json", status = 429)]
pub struct TooManyRequests(Json<Value>);

#[derive(Debug)]
pub struct Ratelimiter {
    key: String,
    reset_after: Duration,
    request_limit: u32,
    request_count: u32,
    last_reset: u64,
}

impl Ratelimiter {
    pub fn new<I>(bucket: &str, identifier: I, limit: u32, reset_after: Duration) -> Ratelimiter
    where
        I: Display,
    {
        Ratelimiter {
            key: format!("ratelimit:{}:{}", identifier, bucket),
            reset_after,
            request_limit: limit,
            request_count: 0,
            last_reset: 0,
        }
    }

    /// Checks if a bucket is ratelimited, if so returns an Error with an ErrorResponse
    pub async fn process_ratelimit(
        &mut self,
        cache: &mut Connection<Cache>,
    ) -> Result<(), RatelimitWrapper<TooManyRequests>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64;

        if let (Some(last_reset), Some(request_count)) = cache
            .hget::<&str, (&str, &str), (Option<u64>, Option<u32>)>(
                &self.key,
                ("last_reset", "request_count"),
            )
            .await
            .expect("Couldn't query cache")
        {
            self.last_reset = last_reset;
            self.request_count = request_count;
            if now - self.last_reset >= self.reset_after.as_millis() as u64 {
                cache
                    .del::<&str, ()>(&self.key)
                    .await
                    .expect("Couldn't query cache");
                cache
                    .hset_multiple::<&str, &str, u64, ()>(
                        &self.key,
                        &[("last_reset", now), ("request_count", 0)],
                    )
                    .await
                    .expect("Couldn't query cache");
                self.last_reset = now;
                self.request_count = 0;
                log::debug!("Reset bucket for {}", self.key);
            }
            if self.request_count >= self.request_limit {
                log::info!("Ratelimited bucket {}", self.key);
                Err(self.wrap_response(TooManyRequests(Json(
                    json!({"status": "429", "msg": "You have been ratelimited", "retry_after": self.last_reset + self.reset_after.as_millis() as u64 - now}),
                ))).unwrap())
            } else {
                cache
                    .hincr::<&str, &str, u8, ()>(&self.key, "request_count", 1)
                    .await
                    .expect("Couldn't query cache");
                self.request_count += 1;
                Ok(())
            }
        } else {
            log::debug!("New bucket for {}", self.key);
            cache
                .hset_multiple::<&str, &str, u64, ()>(
                    &self.key,
                    &[("last_reset", now), ("request_count", 1)],
                )
                .await
                .expect("Couldn't query cache");
            Ok(())
        }
    }

    pub fn wrap_response<T>(
        &self,
        data: T,
    ) -> Result<RatelimitWrapper<T>, RatelimitWrapper<TooManyRequests>> {
        Ok(RatelimitWrapper {
            inner: data,
            ratelimit_reset: Header::new(
                "X-Ratelimit-Reset",
                self.reset_after.as_millis().to_string(),
            ),
            ratelimit_max: Header::new("X-Ratelimit-Max", self.request_limit.to_string()),
            ratelimit_last_reset: Header::new(
                "X-Ratelimit-Last-Reset",
                self.last_reset.to_string(),
            ),
            ratelimit_request_count: Header::new(
                "X-Ratelimit-Request-Count",
                self.request_count.to_string(),
            ),
        })
    }
}

pub struct ClientIP(IpAddr);

impl Display for ClientIP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for ClientIP {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(ip) = req.headers().get_one("CF-Connecting-IP") {
            Outcome::Success(ClientIP(IpAddr::from_str(ip).unwrap()))
        } else {
            Outcome::Success(ClientIP(
                req.client_ip()
                    .unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap()),
            ))
        }
    }
}
