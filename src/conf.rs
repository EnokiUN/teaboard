use std::{env, error::Error, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conf {
    pub name: String,
    pub description: Option<String>,
}

impl Conf {
    pub fn new_from_env() -> Result<Self, Box<dyn Error>> {
        let conf =
            fs::read_to_string(env::var("CONFIG_PATH").unwrap_or_else(|_| "Tea.toml".to_string()))?;
        Ok(toml::from_str(&conf)?)
    }
}
