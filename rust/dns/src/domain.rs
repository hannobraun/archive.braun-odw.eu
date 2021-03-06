use std::{fs::File, io::prelude::*, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
}

impl Domain {
    pub fn load(name: &str) -> anyhow::Result<Self> {
        let name = format!("{}.toml", name);
        let path = Path::new("dns").join(name);

        let mut domain = Vec::new();
        File::open(path)?.read_to_end(&mut domain)?;
        let domain = toml::from_slice(&domain)?;

        Ok(domain)
    }
}
