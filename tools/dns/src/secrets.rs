use serde::Deserialize;

use crate::util::load_toml;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub dns: Dns,
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        let path = "secrets.toml";
        let secrets = load_toml(path)?;
        Ok(secrets)
    }
}

#[derive(Debug, Deserialize)]
pub struct Dns {
    #[serde(rename = "api-token")]
    pub api_token: String,
}
