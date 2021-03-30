use std::path::Path;

use anyhow::Context as _;
use fs_extra::dir::CopyOptions;
use tokio::fs;

pub async fn build(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let output_dir = output_dir.as_ref();

    // TASK: Re-build sites, if contents change.
    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir: {}", output_dir.display())
    })?;
    copy_sites(&output_dir).await.with_context(|| {
        format!(
            "Failed to copy sites to output dir: {}",
            output_dir.display()
        )
    })?;

    Ok(())
}

pub async fn prepare_output_dir(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        fs::remove_dir_all(path).await?;
    }
    fs::create_dir_all(path).await?;

    Ok(())
}

pub async fn copy_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // TASK: Make asynchronous. Probably requires a manual implementation,
    //       possibly based on https://crates.io/crates/async-walkdir.
    fs_extra::copy_items(&["sites"], output_dir, &CopyOptions::new())?;
    Ok(())
}
