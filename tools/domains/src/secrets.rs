use anyhow::Context as _;
use serde::Deserialize;

use crate::util::load_toml;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub domains: Domains,
    // TASK: Add field for private SSH key.
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        let path = "secrets.toml";
        let secrets = load_toml(path).with_context(|| {
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
