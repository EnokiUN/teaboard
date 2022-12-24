use std::time::Duration;

use rocket::{form::Form, http::Status, serde::json::Json, tokio::sync::Mutex, State};
use rocket_db_pools::Connection;
use serde_json::Value;

use crate::{
    auth::PasswordAuth,
    id::IdGen,
    models::{Board, Post, PostForm},
    ratelimit::{ClientIP, Ratelimiter, Response},
    Cache, DB,
};

#[post("/<board>/new", data = "<post>")]
pub async fn new<'a>(
    board: &str,
    post: Form<PostForm<'a>>,
    gen: &State<Mutex<IdGen>>,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    ip: ClientIP,
    auth: PasswordAuth,
) -> Response<Result<Json<Post>, (Status, Json<Value>)>> {
    let mut ratelimiter = Ratelimiter::new("create-post", ip, 1, Duration::from_secs(30));
    ratelimiter.process_ratelimit(&mut cache).await?;
    let board = match Board::get(board, &mut db).await {
        Ok(board) => board,
        Err(err) => return ratelimiter.wrap_response(Err((Status::NotFound, err.0))),
    };
    ratelimiter.wrap_response(
        Post::create(board, post.into_inner(), gen.inner(), &mut db, auth.admin)
            .await
            .map(Json),
    )
}
