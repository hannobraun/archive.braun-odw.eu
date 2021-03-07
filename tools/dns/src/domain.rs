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
        let base = Path::new("dns");
        let toml = base.join(format!("{}.toml", name));

        let domain = util::load_toml(toml)?;
        // TASK: Load zone file.

        Ok(domain)
    }
}
