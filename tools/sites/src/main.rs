use std::{fs, path::Path};

use fs_extra::dir::CopyOptions;

fn main() -> anyhow::Result<()> {
    let output_dir = Path::new("output");

    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir)?;
    fs_extra::copy_items(&["sites"], output_dir, &CopyOptions::new())?;

    Ok(())
}
