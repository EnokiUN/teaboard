mod board;
mod image;
mod post;

use crate::conf::Conf;

pub use self::image::{FetchResponse, Image};
pub use board::Board;
pub use post::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    #[serde(flatten)]
    pub info: Conf,
    pub boards: Vec<Board>,
}
