use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::sqlx::{pool::PoolConnection, MySql};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub description: Option<String>,
}

impl Board {
    pub async fn get(
        id: String,
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
}
