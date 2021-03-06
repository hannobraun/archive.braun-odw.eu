use std::{fs::File, io::Read};

use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    println!("API token: {}", secrets.dns.api_token);
    Ok(())
}

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
