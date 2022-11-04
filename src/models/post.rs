use serde::{Deserialize, Serialize};

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
