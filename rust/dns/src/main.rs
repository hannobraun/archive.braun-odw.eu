use std::{fs::File, io::Read};

use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let mut secrets = Vec::new();
    File::open("secrets.toml")?.read_to_end(&mut secrets)?;
    let secrets: Secrets = toml::from_slice(&secrets)?;

    println!("{:?}", secrets);

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub dns: Dns,
}

#[derive(Debug, Deserialize)]
pub struct Dns {
    #[serde(rename = "api-token")]
    pub api_token: String,
}
