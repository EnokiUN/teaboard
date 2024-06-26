#[macro_use]
extern crate rocket;

mod auth;
mod conf;
mod cors;
mod id;
mod models;
mod ratelimit;
mod routes;

use std::env;

use anyhow::Context;
use id::IdGen;

use rocket::{
    data::{Limits, ToByteUnit},
    tokio::sync::Mutex,
    Config,
};
use rocket_db_pools::{deadpool_redis::Pool, sqlx::SqlitePool, Database};

#[derive(Database)]
#[database("db")]
pub struct DB(SqlitePool);

#[derive(Database)]
#[database("cache")]
pub struct Cache(Pool);

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = Config::figment()
        .merge((
            "limits",
            Limits::default()
                .limit("data-form", 20.mebibytes())
                .limit("file", 20.mebibytes()),
        ))
        .merge(("temp_dir", "./data"))
        .merge((
            "databases.db",
            rocket_db_pools::Config {
                url: env::var("DATABASE_URL")
                    .context("Could not find \"DATABASE_URL\" environment variable")?
                    .strip_prefix("sqlite:")
                    .context("Failed to strip prefix from \"DATABASE_URL\" environment variable")?
                    .to_string(),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ))
        .merge((
            "databases.cache",
            rocket_db_pools::Config {
                url: env::var("REDIS_URL")
                    .context("Could not find \"REDIS_URL\" environment variable")?,
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ));

    let _ = rocket::custom(config)
        .manage(Mutex::new(IdGen::new()))
        .manage(conf::Conf::new_from_env()?)
        .attach(DB::init())
        .attach(Cache::init())
        .attach(cors::Cors)
        .mount("/", routes::routes())
        .launch()
        .await
        .context("Failed to start rest API")?;

    Ok(())
}
