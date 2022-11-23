use std::{fs, path::PathBuf};

use rocket::{
    fs::TempFile,
    http::{ContentType, Header, Status},
    response::status::NotFound,
    serde::json::Json,
};
use serde_json::{json, Value};
use sqlx::{pool::PoolConnection, MySql};
use tokio::{fs::File, sync::Mutex};

use crate::id::IdGen;

pub struct Image {
    pub id: u64,
    pub file_id: u64,
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
    pub async fn create<'a>(
        mut image: TempFile<'a>,
        gen: &Mutex<IdGen>,
        db: &mut PoolConnection<MySql>,
    ) -> Result<Image, (Status, Json<Value>)> {
        let id = gen.lock().await.generate();
        let path = PathBuf::from(format!("./data/{}", id));
        let name = image.name().unwrap().to_string();
        image.persist_to(&path).await.unwrap();
        let data = tokio::fs::read(&path).await.unwrap();

        let hash = sha256::digest(&data[..]);
        let img = if let Ok(img) = sqlx::query_as!(
            Image,
            "
SELECT *
FROM images
WHERE hash = ?
            ",
            hash,
        )
        .fetch_one(&mut *db)
        .await
        {
            tokio::fs::remove_file(path).await.unwrap();
            sqlx::query!(
                "
INSERT INTO images(id, file_id, name, content_type, hash)
VALUES(?, ?, ?, ?, ?)
                ",
                id,
                img.id,
                name,
                img.content_type,
                hash
            )
            .execute(&mut *db)
            .await
            .unwrap();

            Image {
                id,
                file_id: img.id,
                name,
                content_type: img.content_type,
                hash,
            }
        } else {
            let img = tokio::task::spawn_blocking(move || {
                let mime = tree_magic::from_u8(&data);
                match mime.as_ref() {
                    "image/gif" | "image/jpeg" | "image/png" => {
                        let metadata = rexiv2::Metadata::new_from_buffer(&data).unwrap();
                        metadata.clear();
                        metadata.save_to_file(path).unwrap();
                    },
                    "image/webp" | "video/mp4" | "video/webm" | "video/quicktime" => {}
                    _ => {
                        fs::remove_dir(path).unwrap();
                        return Err((Status::BadRequest, Json(json!({"status": 400, "msg": "Only major image and video formats are supported"}))))
                    }
                }
                Ok(Image {
                    id,
                    file_id: id,
                    name,
                    content_type: mime,
                    hash
                })
            })
            .await
            .unwrap()?;
            sqlx::query!(
                "
INSERT INTO images(id, file_id, name, content_type, hash)
VALUES(?, ?, ?, ?, ?)
                ",
                img.id,
                img.id,
                img.name,
                img.content_type,
                img.hash
            )
            .execute(&mut *db)
            .await
            .unwrap();

            img
        };

        Ok(img)
    }

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
