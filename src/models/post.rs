use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: u64,
    board: String,
    title: String,
    content: String,
    pinned: bool,
    moderator: bool,
    locked: bool,
    parent: u64,
    image: u64,
}
