mod args;

use std::{fs, path::Path};

use clap::Clap as _;
use fs_extra::dir::CopyOptions;

use self::args::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output_dir = Path::new("output");

    // TASK: Re-build sites, if contents change.
    prepare_output_dir(output_dir)?;
    copy_sites(output_dir)?;

    if args.serve {
        serve_sites(output_dir).await?;
    }

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

async fn serve_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let sites_dir = output_dir.as_ref().join("sites");

    let filter = warp::filters::fs::dir(sites_dir);
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
