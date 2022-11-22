use rocket::{
    http::{ContentType, Header},
    response::status::NotFound,
    serde::json::Json,
};
use serde_json::{json, Value};
use sqlx::{pool::PoolConnection, MySql};
use tokio::fs::File;

pub struct Image {
    pub id: u64,
    pub name: String,
    pub content_type: String,
    pub hash: String,
}

#[derive(Debug, Responder)]
pub struct FetchResponse<'a> {
    file: File,
    disposition: Header<'a>,
    content_type: ContentType,
}

impl Image {
    pub async fn get<'a>(
        id: u64,
        db: &mut PoolConnection<MySql>,
    ) -> Result<FetchResponse<'a>, NotFound<Json<Value>>> {
        let img = sqlx::query_as!(
            Image,
            "
SELECT *
FROM images
WHERE id = ?
            ",
            id
        )
        .fetch_one(db)
        .await
        .map_err(|_| NotFound(Json(json!({"status": 400, "msg": "Image not found"}))))?;
        let file = File::open(format!("data/{}", img.id)).await.unwrap();
        Ok(FetchResponse {
            file,
            disposition: Header::new(
                "Content-Disposition",
                format!("inline; filename=\"{}\"", img.name),
            ),
            content_type: ContentType::parse_flexible(&img.content_type).unwrap(),
        })
    }
}
