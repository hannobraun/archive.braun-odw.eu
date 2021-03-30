mod args;

use std::{fs, path::Path};

use clap::Clap as _;
use fs_extra::dir::CopyOptions;

use self::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("serve: {}", args.serve);

    let output_dir = Path::new("output");

    prepare_output_dir(output_dir)?;
    copy_sites(output_dir)?;

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

fn copy_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    fs_extra::copy_items(&["sites"], output_dir, &CopyOptions::new())?;
    Ok(())
}
