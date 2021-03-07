use std::path::Path;

use serde::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
    // TASK: Add field for zone file, but don't deserialize it with Serde. Load
    //       it manually.
}

impl Domain {
    pub fn load(name: &str) -> anyhow::Result<Self> {
        let name = format!("{}.toml", name);
        let path = Path::new("dns").join(name);

        let domain = util::load_toml(path)?;
        Ok(domain)
    }
}
