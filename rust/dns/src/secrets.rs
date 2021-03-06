use std::{fs::File, io::prelude::*};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub dns: Dns,
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        let mut secrets = Vec::new();
        File::open("secrets.toml")?.read_to_end(&mut secrets)?;
        let secrets = toml::from_slice(&secrets)?;

        Ok(secrets)
    }
}

#[derive(Debug, Deserialize)]
pub struct Dns {
    #[serde(rename = "api-token")]
    pub api_token: String,
}