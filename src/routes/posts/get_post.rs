use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{models::Post, DB};

#[get("/posts/<id>")]
pub async fn get(id: u64, mut db: Connection<DB>) -> Result<Json<Post>, NotFound<Json<Value>>> {
    Post::get(id, &mut *db).await.map(|p| Json(p))
}
