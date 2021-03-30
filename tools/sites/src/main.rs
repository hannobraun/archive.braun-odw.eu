use std::{fs, path::Path};

use fs_extra::dir::CopyOptions;

fn main() -> anyhow::Result<()> {
    let output_dir = Path::new("output");

    prepare_output_dir(output_dir)?;
    fs_extra::copy_items(&["sites"], output_dir, &CopyOptions::new())?;

    Ok(())
}

fn prepare_output_dir(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        fs::remove_dir_all(path)?;
    }
    fs::create_dir_all(path)?;

    Ok(())
}
