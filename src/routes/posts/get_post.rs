use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{Post, PostInfo},
    DB,
};

#[get("/posts/<id>")]
pub async fn get(id: u64, mut db: Connection<DB>) -> Result<Json<PostInfo>, NotFound<Json<Value>>> {
    Post::info(id, &mut *db).await.map(|p| Json(p))
}
