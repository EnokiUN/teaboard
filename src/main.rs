#[macro_use]
extern crate rocket;

mod conf;
mod cors;
mod id;
mod models;
mod ratelimit;
mod routes;

use std::env;

use id::IdGen;

use rocket::{
    data::{Limits, ToByteUnit},
    tokio::sync::Mutex,
    Build, Config, Rocket,
};
use rocket_db_pools::{deadpool_redis::Pool, sqlx::MySqlPool, Database};

#[derive(Database)]
#[database("db")]
pub struct DB(MySqlPool);

#[derive(Database)]
#[database("cache")]
pub struct Cache(Pool);

#[launch]
async fn launch() -> Rocket<Build> {
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
                    .expect("Could not find \"DATABASE_URL\" environment variable"),
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
                    .expect("Could not find \"REDIS_URL\" environment variable"),
                min_connections: None,
                max_connections: 1024,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ));

    rocket::custom(config)
        .manage(Mutex::new(IdGen::new()))
        .manage(conf::Conf::new_from_env().expect("Could not read instance config"))
        .attach(DB::init())
        .attach(Cache::init())
        .attach(cors::Cors)
        .mount("/", routes::routes())
}
