use std::path::Path;

use anyhow::Context as _;
use futures::StreamExt as _;
use tokio::fs;
use tracing::debug;

use super::walk::walk_dir;

pub async fn copy(
    source_dir: &Path,
    output_dir: &Path,
) -> anyhow::Result<()> {
    let source_dir = source_dir.join("static");
    let output_dir = output_dir.to_path_buf();

    let mut entries = walk_dir(source_dir, output_dir);
    while let Some(entry) = entries.next().await {
        let (source, output) = entry?;

        debug!("Copying `{}` to `{}`", source.display(), output.display());

        copy_dir_entry(&source, &output).await.with_context(|| {
            format!("Failed to copy directory entry: {}", source.display())
        })?;
    }

    Ok(())
}

async fn copy_dir_entry(source: &Path, output: &Path) -> anyhow::Result<()> {
    if source.is_dir() {
        fs::create_dir_all(output).await?;
    } else {
        fs::copy(source, output).await?;
    }

    Ok(())
}
