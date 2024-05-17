use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, Sqlite};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{Post, PostInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub description: Option<String>,
}

impl Board {
    pub async fn get(
        id: &str,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, NotFound<Json<Value>>> {
        sqlx::query_as!(
            Board,
            "
SELECT *
FROM boards
WHERE id = ?",
            id
        )
        .fetch_one(&mut **db)
        .await
        .map_err(|_| NotFound(Json(json!({"status": 404, "msg": "Unknown board"}))))
    }

    pub async fn all(db: &mut PoolConnection<Sqlite>) -> Vec<Board> {
        sqlx::query_as!(
            Self,
            "
SELECT * FROM boards
            "
        )
        .fetch_all(&mut **db)
        .await
        .unwrap()
    }

    pub async fn get_feed(
        id: &str,
        before: Option<i64>,
        limit: Option<u32>,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<PostInfo>, NotFound<Json<Value>>> {
        let board = Self::get(id, &mut *db).await?;
        let limit = limit.unwrap_or(10);
        let posts: Vec<i64> = match before {
            Some(before) => sqlx::query!(
                r#"
SELECT id
FROM posts
WHERE board = ?
AND id < ?
AND parent IS NULL
ORDER BY id DESC
LIMIT ?
                    "#,
                board.id,
                before,
                limit
            )
            .fetch_all(&mut **db)
            .await
            .unwrap()
            .iter()
            .map(|p| p.id)
            .collect(),
            None => sqlx::query!(
                r#"
SELECT id
FROM posts
WHERE board = ?
AND parent IS NULL
ORDER BY id DESC
LIMIT ?
                    "#,
                board.id,
                limit
            )
            .fetch_all(&mut **db)
            .await
            .unwrap()
            .iter()
            .map(|p| p.id)
            .collect(),
        };
        let mut infos: Vec<PostInfo> = Vec::with_capacity(posts.len());
        for id in posts.into_iter() {
            infos.push(Post::partial_info(id, db).await?)
        }
        Ok(infos)
    }

    pub async fn create(data: Self, db: &mut PoolConnection<Sqlite>) -> Self {
        sqlx::query!(
            "
INSERT INTO boards(id, description)
VALUES(?, ?)
            ",
            data.id,
            data.description,
        )
        .execute(&mut **db)
        .await
        .unwrap();
        data
    }

    pub async fn edit(
        id: &str,
        description: Option<String>,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<(), NotFound<Json<Value>>> {
        Self::get(id, &mut *db).await?;
        sqlx::query!(
            "
UPDATE boards
SET description = ?
WHERE id = ?
            ",
            description,
            id
        )
        .execute(&mut **db)
        .await
        .unwrap();
        Ok(())
    }

    pub async fn delete(
        id: &str,
        db: &mut PoolConnection<Sqlite>,
    ) -> Result<(), NotFound<Json<Value>>> {
        Self::get(id, &mut *db).await?;
        sqlx::query!(
            "
DELETE FROM posts
WHERE board = ?
            ",
            id
        )
        .execute(&mut **db)
        .await
        .unwrap();
        sqlx::query!(
            "
DELETE FROM boards
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
