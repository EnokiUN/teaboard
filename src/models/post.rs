use rocket::{fs::TempFile, response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, MySql};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use tokio::sync::Mutex;

use crate::id::IdGen;

use super::Board;

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
    #[serde_as(as = " Option<DisplayFromStr>")]
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
    ) -> Result<Post, NotFound<Json<Value>>> {
        let _image = form.image;
        let post = form.post.into_inner();
        let id = gen.lock().await.generate();
        if let Some(parent) = post.parent {
            Self::get(parent, db)
                .await
                .map_err(|_| NotFound(Json(json!({"code": 404, "msg": "Unknown parent post"}))))?;
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
            None::<u32>
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
            image: None,
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
}
