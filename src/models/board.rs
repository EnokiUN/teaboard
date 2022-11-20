use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, MySql};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::Post;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub description: Option<String>,
}

impl Board {
    pub async fn get(
        id: &str,
        db: &mut PoolConnection<MySql>,
    ) -> Result<Self, NotFound<Json<Value>>> {
        sqlx::query_as!(
            Board,
            "
SELECT *
FROM boards
WHERE id = ?",
            id
        )
        .fetch_one(db)
        .await
        .map_err(|_| NotFound(Json(json!({"code": 404, "msg": "Unknown board"}))))
    }
    pub async fn get_feed(
        id: String,
        before: Option<u64>,
        limit: Option<u32>,
        db: &mut PoolConnection<MySql>,
    ) -> Result<Vec<Post>, NotFound<Json<Value>>> {
        let board = Self::get(&id, db).await?;
        let limit = limit.unwrap_or(10);
        let posts = match before {
            Some(before) => 
                sqlx::query_as!(
                    Post,
                    r#"
SELECT id, board, title, content, pinned as "pinned: _", moderator as "moderator: _", locked as "locked: _", parent, image
FROM posts
WHERE board = ?
AND id < ?
ORDER BY id DESC
LIMIT ?
                    "#,
                    board.id,
                    before,
                    limit
                    )
                    .fetch_all(db)
                    .await
                    .unwrap(),
            None =>
                sqlx::query_as!(
                    Post,
                    r#"
SELECT id, board, title, content, pinned as "pinned: _", moderator as "moderator: _", locked as "locked: _", parent, image
FROM posts
WHERE board = ?
ORDER BY id DESC
LIMIT ?
                    "#,
                    board.id,
                    limit
                    )
                    .fetch_all(db)
                    .await
                    .unwrap(),
        };
        Ok(posts)
    }
}
