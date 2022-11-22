use std::{fs, path::PathBuf};

use rocket::{fs::TempFile, http::Status, response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, MySql};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use tokio::sync::Mutex;

use crate::id::IdGen;

use super::{Board, Image};

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde_as(as = "DisplayFromStr")]
    pub id: u64,
    pub board: String,
    pub title: String,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub pinned: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub moderator: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub locked: bool,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub parent: Option<u64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub image: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostInfo {
    #[serde(flatten)]
    pub post: Post,
    pub replies: Vec<Post>,
}

fn is_false(foo: &bool) -> bool {
    !foo
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    pub title: String,
    pub content: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub parent: Option<u64>,
}

#[derive(Debug, FromForm)]
pub struct PostForm<'a> {
    pub post: Json<PostJson>,
    pub image: Option<TempFile<'a>>,
}

impl Post {
    pub async fn create<'a>(
        board: Board,
        form: PostForm<'a>,
        gen: &Mutex<IdGen>,
        db: &mut PoolConnection<MySql>,
    ) -> Result<Post, (Status, Json<Value>)> {
        let image: Option<u64> = match form.image {
            Some(mut image) => {
                let id = gen.lock().await.generate();
                let path = PathBuf::from(format!("./data/{}", id));
                let name = image.name().unwrap().to_string();
                image.persist_to(&path).await.unwrap();

                let hash = sha256::try_digest(path.as_path()).unwrap();
                if let Ok(img) = sqlx::query_as!(
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
                    Some(img.id)
                } else {
                    let img = tokio::task::spawn_blocking(move || {
                        let mime = tree_magic::from_filepath(path.as_path());
                        match mime.as_ref() {
                            "image/gif" | "image/jpeg" | "image/png" | "image/webp" | "video/mp4" | "video/webm" | "video/quicktime" => {}
                            _ => {
                                fs::remove_dir(path).unwrap();
                                return Err((Status::BadRequest, Json(json!({"status":400,"msg":"Only major image and video formats are supported"}))))
                            }
                        }
                        Ok(Image {
                            id,
                            name,
                            content_type: mime,
                            hash
                        })
                    })
                    .await
                    .unwrap()?;
                    sqlx::query!(
                        "
INSERT INTO images(id, name, content_type, hash)
VALUES(?, ?, ?, ?)
                        ",
                        img.id,
                        img.name,
                        img.content_type,
                        img.hash
                    )
                    .execute(&mut *db)
                    .await
                    .unwrap();

                    Some(img.id)
                }
            }
            None => None,
        };
        let post = form.post.into_inner();
        let id = gen.lock().await.generate();
        if let Some(parent) = post.parent {
            Self::get(parent, db).await.map_err(|_| {
                (
                    Status::NotFound,
                    Json(json!({"code": 404, "msg": "Unknown parent post"})),
                )
            })?;
        }
        sqlx::query!(
            "
INSERT INTO posts(id, board, title, content, parent, image)
VALUES(?, ?, ?, ?, ?, ?)
            ",
            id,
            board.id,
            post.title,
            post.content,
            post.parent,
            image
        )
        .execute(db)
        .await
        .unwrap();
        Ok(Self {
            id,
            board: board.id,
            title: post.title,
            content: post.content,
            pinned: false,
            moderator: false,
            locked: false,
            parent: post.parent,
            image,
        })
    }

    pub async fn get(
        id: u64,
        db: &mut PoolConnection<MySql>,
    ) -> Result<Post, NotFound<Json<Value>>> {
        // https://github.com/launchbadge/sqlx/issues/877
        sqlx::query_as!(
            Self,
            r#"
SELECT id, board, title, content, pinned as "pinned: _", moderator as "moderator: _", locked as "locked: _", parent, image
FROM posts
WHERE id = ?
            "#,
            id
        )
        .fetch_one(db)
        .await
        .map_err(|_| NotFound(Json(json!({"code": 404, "msg": "Unknown post"}))))
    }

    pub async fn info(
        id: u64,
        db: &mut PoolConnection<MySql>,
    ) -> Result<PostInfo, NotFound<Json<Value>>> {
        let post = Self::get(id, db).await?;
        let replies = sqlx::query_as!(
            Self,
            r#"
SELECT id, board, title, content, pinned as "pinned: _", moderator as "moderator: _", locked as "locked: _", parent, image
FROM posts
WHERE parent = ?
            "#,
            id
        )
        .fetch_all(db)
        .await
        .map_err(|_| NotFound(Json(json!({"code": 404, "msg": "Unknown post"}))))?;
        Ok(PostInfo { post, replies })
    }

    pub async fn partial_info(
        id: u64,
        db: &mut PoolConnection<MySql>,
    ) -> Result<PostInfo, NotFound<Json<Value>>> {
        let post = Self::get(id, db).await?;
        let replies = sqlx::query_as!(
            Self,
            r#"
SELECT id, board, title, content, pinned as "pinned: _", moderator as "moderator: _", locked as "locked: _", parent, image
FROM posts
WHERE parent = ?
ORDER BY id DESC
LIMIT 5
            "#,
            id
        )
        .fetch_all(db)
        .await
        .map_err(|_| NotFound(Json(json!({"code": 404, "msg": "Unknown post"}))))?;
        Ok(PostInfo { post, replies })
    }
}
