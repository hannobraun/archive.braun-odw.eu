use std::path::Path;

use serde::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,

    /// The contents of the zone file
    ///
    /// This isn't deserialized from the TOML file using Serde, but instead
    /// loaded manually in [`Domain::load`].
    #[serde(skip)]
    pub zone: String,
}

impl Domain {
    pub fn load(name: &str) -> anyhow::Result<Self> {
        let name = format!("{}.toml", name);
        let path = Path::new("dns").join(name);

        let domain = util::load_toml(path)?;
        // TASK: Load zone file.

        Ok(domain)
    }
}
