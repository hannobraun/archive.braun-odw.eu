use std::{fs::File, io::prelude::*, path::Path};

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
    // TASK: Load all domains from a given directory instead.
    pub fn load(name: &str) -> anyhow::Result<Self> {
        let base = Path::new("domains");
        let toml = base.join(format!("{}.toml", name));
        let zone = base.join(format!("{}.zone", name));

        let mut domain: Self = util::load_toml(toml)?;
        File::open(zone)?.read_to_string(&mut domain.zone)?;

        Ok(domain)
    }
}
