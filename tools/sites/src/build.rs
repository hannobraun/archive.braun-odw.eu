use std::{fs, path::Path};

use fs_extra::dir::CopyOptions;

pub fn prepare_output_dir(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;

    Ok(())
}

pub fn copy_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    fs_extra::copy_items(&["sites"], output_dir, &CopyOptions::new())?;
    Ok(())
}
