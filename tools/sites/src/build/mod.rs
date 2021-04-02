mod html;
mod transform;
mod walk;
mod watch;

pub use self::transform::Transform;

use std::path::Path;

use anyhow::Context as _;
use futures::StreamExt as _;
use tokio::fs;
use tracing::{debug, info};

use self::walk::walk_dir;

pub async fn build_continuously(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    transform: &mut impl Transform,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();

    // Build at least once, before waiting for events.
    info!("Building sites.");
    build(source_dir, &output_dir, transform).await?;

    let mut watcher = watch::Watcher::new(source_dir)?;
    while let Some(trigger) = watcher.watch().await? {
        info!("Building sites. Trigger: {}", trigger);
        build(source_dir, &output_dir, transform).await?;
    }

    Ok(())
}

pub async fn build(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    transform: &mut impl Transform,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir: {}", output_dir.display())
    })?;
    copy_static(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir: {}",
                output_dir.display()
            )
        })?;
    html::build_html(&source_dir, &output_dir, transform)
        .await
        .context("Failed to build HTML files")?;

    Ok(())
}

async fn prepare_output_dir(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).await?;
    }
    fs::create_dir_all(path).await?;

    Ok(())
}

async fn copy_static(
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
