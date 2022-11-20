use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub description: Option<String>,
}
