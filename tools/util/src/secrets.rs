use std::collections::HashMap;

use anyhow::Context as _;
use serde::Deserialize;

use crate::toml;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub domains: Domains,
    pub nodes: HashMap<String, Node>,
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        let path = "secrets.toml";
        let secrets = toml::load(path).with_context(|| {
            format!("Failed to load secrets from `{}`", path)
        })?;
        Ok(secrets)
    }
}

#[derive(Debug, Deserialize)]
pub struct Domains {
    #[serde(rename = "api-token")]
    pub api_token: String,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    #[serde(rename = "ssh-key")]
    pub ssh_key: String,
}
