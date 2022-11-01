#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[launch]
async fn launch() -> Rocket<Build> {
    dotenvy::dotenv().ok();
    env_logger::init();

    rocket::build()
}
