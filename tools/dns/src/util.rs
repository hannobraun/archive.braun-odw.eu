use std::{fs::File, io::prelude::*, path::Path};

use serde::de::DeserializeOwned;

pub fn load_toml<T: DeserializeOwned>(path: impl AsRef<Path>) -> anyhow::Result<T> {
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;

    let toml = toml::from_slice(&buf)?;
    Ok(toml)
}
