use std::{env, fs};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conf {
    pub name: String,
    pub description: Option<String>,
    pub password: String,
}

impl Conf {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let conf =
            fs::read_to_string(env::var("CONFIG_PATH").unwrap_or_else(|_| "Tea.toml".to_string()))?;
        let conf: Conf = toml::from_str(&conf).context("Could not deserialize toml file")?;
        if conf.name.is_empty() {
            bail!("Instance name cannot be empty");
        }
        if conf.name.len() > 32 {
            bail!("Instance name cannot be more than 32 characters long");
        }
        if let Some(description) = &conf.description {
            if description.is_empty() {
                bail!("Instance description cannot be empty");
            }
            if description.len() > 2000 {
                bail!("Instance description cannot be more than 2000 characters long");
            }
        }
        Ok(conf)
    }
}
