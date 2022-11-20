use rocket::{fs::TempFile, response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, MySql};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::id::IdGen;

use super::Board;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub board: String,
    pub title: String,
    pub content: Option<String>,
    pub pinned: bool,
    pub moderator: bool,
    pub locked: bool,
    pub parent: Option<u64>,
    pub image: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    pub title: String,
    pub content: Option<String>,
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
    ) -> Self {
        let _image = form.image;
        let post = form.post.into_inner();
        let id = gen.lock().await.generate();
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
        Self {
            id,
            board: board.id,
            title: post.title,
            content: post.content,
            pinned: false,
            moderator: false,
            locked: false,
            parent: post.parent,
            image: None,
        }
    }

}
