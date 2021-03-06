use std::{fs::File, io::prelude::*};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
}

impl Domain {
    pub fn load() -> anyhow::Result<Self> {
        let mut domain = Vec::new();
        File::open("dns/braun-odw.eu.toml")?.read_to_end(&mut domain)?;
        let domain = toml::from_slice(&domain)?;

        Ok(domain)
    }
}
