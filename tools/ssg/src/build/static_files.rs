use std::path::Path;

use anyhow::Context as _;
use futures::StreamExt as _;
use tokio::fs;
use tracing::{debug, info};

use super::walk::walk_dir;

pub async fn copy(source_dir: &Path, output_dir: &Path) -> anyhow::Result<()> {
    let source_dir = source_dir.join("static");
    let output_dir = output_dir.to_path_buf();

    info!(
        "Copying static files (`{}` to `{}`)",
        source_dir.display(),
        output_dir.display()
    );

    let mut entries = walk_dir(source_dir, output_dir);
    while let Some(entry) = entries.next().await {
        let (source, output) = entry?;

        debug!("Copying `{}` to `{}`", source.display(), output.display());

        copy_dir_entry(&source, &output).await.with_context(|| {
            format!(
                "Failed to copy `{}` to `{}`",
                source.display(),
                output.display(),
            )
        })?;
    }

    Ok(())
}

async fn copy_dir_entry(source: &Path, output: &Path) -> anyhow::Result<()> {
    if source.is_dir() {
        fs::create_dir_all(output).await?;
    } else {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::copy(source, output).await?;
    }

    Ok(())
}
