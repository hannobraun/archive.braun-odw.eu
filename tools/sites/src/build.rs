use std::path::Path;

use anyhow::Context as _;
use async_walkdir::WalkDir;
use futures::StreamExt as _;
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
    let source_dir = Path::new("sites");
    let output_dir = output_dir.as_ref();

    let mut entries = WalkDir::new(source_dir);

    while let Some(entry) = entries.next().await {
        let source = entry?.path();
        let output = output_dir.join(&source);

        handle_entry(&source, &output).await.with_context(|| {
            format!("Failed to handle directory entry: {}", source.display())
        })?;
    }

    Ok(())
}

async fn handle_entry(source: &Path, output: &Path) -> anyhow::Result<()> {
    if source.is_dir() {
        fs::create_dir_all(output).await?;
    } else {
        fs::copy(source, output).await?;
    }

    Ok(())
}
