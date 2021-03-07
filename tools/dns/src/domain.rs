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
        let toml = format!("{}.toml", name);
        let toml = Path::new("dns").join(toml);

        let domain = util::load_toml(toml)?;
        // TASK: Load zone file.

        Ok(domain)
    }
}
