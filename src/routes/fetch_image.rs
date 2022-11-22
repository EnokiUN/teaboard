use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    models::{FetchResponse, Image},
    DB,
};

#[get("/images/<id>")]
pub async fn fetch_image<'a>(
    id: u64,
    mut db: Connection<DB>,
) -> Result<FetchResponse<'a>, NotFound<Json<Value>>> {
    Image::get(id, &mut *db).await
}
