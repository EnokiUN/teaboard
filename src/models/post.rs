#![allow(clippy::unnecessary_lazy_evaluations)] // FormForm macro does this, nothing I can do about
                                                // it sadly
use lazy_static::lazy_static;
use regex::Regex;
use rocket::{fs::TempFile, http::Status, response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, Sqlite};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use sqlx::{sqlite::SqliteRow, FromRow, Row};
use tokio::sync::Mutex;

use crate::id::IdGen;

use super::{Board, Image};

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
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
    pub parent: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub image: Option<i64>,
}

impl FromRow<'_, SqliteRow> for Post {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            board: row.get("board"),
            title: row.get("title"),
            content: row.get("content"),
            pinned: row.get("pinned"),
            moderator: row.get("moderator"),
            locked: row.get("locked"),
            parent: row.get("parent"),
            image: row.get("image"),
        })
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PostInfo {
    #[serde(flatten)]
    pub post: Post,
    pub replies: Vec<Post>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub mentions: Vec<i64>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub mentioned_posts: Vec<i64>,
}

fn is_false(value: &bool) -> bool {
    !value
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    pub title: String,
    pub content: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub parent: Option<i64>,
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
        db: &mut PoolConnection<Sqlite>,
        moderator: bool,
    ) -> Result<Post, (Status, Json<Value>)> {
        lazy_static! {
            pub static ref MENTION_REGEX: Regex = Regex::new(r">>(\d{9,12})").unwrap();
        }
        let image: Option<i64> = match form.image {
            Some(image) => Some(Image::create(image, gen, &mut *db).await?.id),
            None => None,
        };
        let post = form.post.into_inner();
        let id = gen.lock().await.generate();
        if let Some(parent) = post.parent {
            let parent = Self::get(parent, db).await.map_err(|_| {
                (
                    Status::NotFound,
                    Json(json!({"status": 404, "msg": "Unknown parent post"})),
                )
            })?;
            if parent.board != board.id {
                return Err((
                    Status::BadRequest,
                    Json(
                        json!({"status": 400, "msg": "You can't reply to a post on another board"}),
                    ),
                ));
            }
            if parent.locked && !moderator {
                return Err((
                    Status::Forbidden,
                    Json(json!({"status": 403, "msg": "This post is already locked"})),
                ));
            }
        }
        sqlx::query!(
            "
INSERT INTO posts(id, board, title, content, moderator, parent, image)
VALUES(?, ?, ?, ?, ?, ?, ?)
            ",
            id,
            board.id,
            post.title,
            post.content,
            moderator,
            post.parent,
            image,
        )
        .execute(&mut **db)
        .await
        .unwrap();
        if let Some(content) = &post.content {
            for capture in MENTION_REGEX.captures_iter(content) {
                if let Ok(post) =
                    Self::get(capture.get(1).unwrap().as_str().parse().unwrap(), &mut *db).await
                {
                    sqlx::query!(
                        "
INSERT INTO mentions(post, mentioned_post)
VALUES(?, ?)
                        ",
                        post.id,
                        id
                    )
                    .execute(&mut **db)
                    .await
                    .unwrap();
                }
            }
        }
        Ok(Self {
            id,
            board: board.id,
            title: post.title,
            content: post.content,
            pinned: false,
            moderator,
            locked: false,
            parent: post.parent,
            image,
        })
    }

    pub async fn get(
        id: i64,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<Post, NotFound<Json<Value>>> {
        // https://github.com/launchbadge/sqlx/issues/877
        sqlx::query_as(
            r#"
SELECT *
FROM posts
WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .map_err(|_| NotFound(Json(json!({"status": 404, "msg": "Unknown post"}))))
    }

    pub async fn info(
        id: i64,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<PostInfo, NotFound<Json<Value>>> {
        let post = Self::get(id, db).await?;
        let replies = sqlx::query_as(
            r#"
SELECT *
FROM posts
WHERE parent = ?
ORDER BY id DESC
            "#,
        )
        .bind(id)
        .fetch_all(&mut **db)
        .await
        .map_err(|_| NotFound(Json(json!({"status": 404, "msg": "Unknown post"}))))?;
        let mentions = sqlx::query!(
            "
SELECT post
FROM mentions
WHERE mentioned_post = ?
            ",
            id
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.post)
        .collect();
        let mentioned_posts = sqlx::query!(
            "
SELECT mentioned_post
FROM mentions
WHERE post = ?
            ",
            id
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.mentioned_post)
        .collect();

        Ok(PostInfo {
            post,
            replies,
            mentions,
            mentioned_posts,
        })
    }

    pub async fn partial_info(
        id: i64,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<PostInfo, NotFound<Json<Value>>> {
        let post = Self::get(id, db).await?;
        let replies = sqlx::query_as(
            r#"
SELECT *
FROM posts
WHERE parent = ?
ORDER BY id DESC
LIMIT 5
            "#,
        )
        .bind(id)
        .fetch_all(&mut **db)
        .await
        .map_err(|_| NotFound(Json(json!({"status": 404, "msg": "Unknown post"}))))?;
        let mentions = sqlx::query!(
            "
SELECT post
FROM mentions
WHERE mentioned_post = ?
            ",
            id
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.post)
        .collect();
        let mentioned_posts = sqlx::query!(
            "
SELECT mentioned_post
FROM mentions
WHERE post = ?
            ",
            id
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
        .into_iter()
        .map(|m| m.mentioned_post)
        .collect();

        Ok(PostInfo {
            post,
            replies,
            mentions,
            mentioned_posts,
        })
    }

    pub async fn pin(
        id: i64,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<(), NotFound<Json<Value>>> {
        Self::get(id, db).await?;
        sqlx::query!(
            "
UPDATE posts
SET pinned = TRUE
WHERE id = ?
            ",
            id
        )
        .execute(&mut **db)
        .await
        .unwrap();
        Ok(())
    }

    pub async fn lock(
        id: i64,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<(), NotFound<Json<Value>>> {
        Self::get(id, db).await?;
        sqlx::query!(
            "
UPDATE posts
SET locked = TRUE
WHERE id = ?
            ",
            id
        )
        .execute(&mut **db)
        .await
        .unwrap();
        Ok(())
    }
}
