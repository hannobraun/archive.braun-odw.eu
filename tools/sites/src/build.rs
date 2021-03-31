use std::path::Path;

use anyhow::Context as _;
use async_walkdir::WalkDir;
use futures::StreamExt as _;
use notify::{immediate_watcher, RecommendedWatcher, Watcher};
use tokio::{fs, sync::mpsc::unbounded_channel};
use tracing::info;

pub async fn build_continuously(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();
    let source_dir_abs = source_dir.canonicalize()?;

    // Build at least once, before waiting for events.
    info!("Building sites.");
    build(source_dir, &output_dir).await?;

    let (tx, mut rx) = unbounded_channel();

    let mut watcher: RecommendedWatcher = immediate_watcher(move |event| {
        // The function returns an error, if the received has been closed.
        // This shouldn't happen unless there's a bug, in which case
        // crashing this thread probably isn't the worst idea.
        tx.send(event).unwrap()
    })?;
    watcher.watch(source_dir, notify::RecursiveMode::Recursive)?;

    while let Some(event) = rx.recv().await {
        let event = event?;
        let paths: Vec<_> = event
            .paths
            .into_iter()
            .map(|path| {
                // If we can't strip the prefix, just leave the path as-is.
                let path = path.strip_prefix(&source_dir_abs).unwrap_or(&path);
                path.to_path_buf()
            })
            .collect();

        info!("Building sites. Trigger: {:?} on {:?}", event.kind, paths);

        // Let's not be picky and just rebuild on any event.
        build(source_dir, &output_dir).await?;
    }

    Ok(())
}

pub async fn build(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir: {}", output_dir.display())
    })?;
    copy_sites(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir: {}",
                output_dir.display()
            )
        })?;

    Ok(())
}

async fn prepare_output_dir(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).await?;
    }
    fs::create_dir_all(path).await?;

    Ok(())
}

async fn copy_sites(
    source_dir: &Path,
    output_dir: &Path,
) -> anyhow::Result<()> {
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
