use rocket::{
    form::Form, response::status::NotFound, serde::json::Json, tokio::sync::Mutex, State,
};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    id::IdGen,
    models::{Board, Post, PostForm},
    DB,
};

#[post("/<board>/new", data = "<post>")]
pub async fn new<'a>(
    board: &str,
    post: Form<PostForm<'a>>,
    gen: &State<Mutex<IdGen>>,
    mut db: Connection<DB>,
) -> Result<Json<Post>, NotFound<Json<Value>>> {
    let board = Board::get(board, &mut *db).await?;
    Post::create(board, post.into_inner(), gen.inner(), &mut *db)
        .await
        .map(|p| Json(p))
}
