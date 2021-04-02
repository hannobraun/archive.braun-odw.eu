mod walk;
mod watch;

use std::path::Path;

use anyhow::Context as _;
use futures::StreamExt as _;
use kuchiki::traits::TendrilSink as _;
use tokio::fs;
use tracing::{debug, info};

use self::walk::walk_dir;

pub async fn build_continuously(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();

    // Build at least once, before waiting for events.
    info!("Building sites.");
    build(source_dir, &output_dir).await?;

    let mut watcher = watch::Watcher::new(source_dir)?;
    while let Some(trigger) = watcher.watch().await? {
        info!("Building sites. Trigger: {}", trigger);
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
    copy_static(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir: {}",
                output_dir.display()
            )
        })?;
    build_html(&source_dir, &output_dir)
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

async fn build_html(
    source_dir: &Path,
    output_dir: &Path,
) -> anyhow::Result<()> {
    // TASK: Process all files in `html`, recursively.
    let source_file = "hanno.braun-odw.eu/index.html";

    let source = source_dir.join("html").join(source_file);

    // TASK: Look into `parse_html_with_options` and see which ones might make
    //       a difference.
    let document = kuchiki::parse_html()
        .from_utf8()
        .from_file(&source)
        .with_context(|| {
            format!("Failed to parse HTML file `{}`", source.display())
        })?;

    // TASK: Transform document. Probably best to keep the transformations
    //       themselves out of this function and accept a closure that does them
    //       as an argument.

    document.serialize_to_file(output_dir.join(source_file))?;

    Ok(())
}
