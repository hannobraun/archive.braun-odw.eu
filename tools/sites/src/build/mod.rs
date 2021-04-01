mod watch;

use std::{fmt::Write as _, path::Path};

use anyhow::Context as _;
use async_walkdir::WalkDir;
use futures::StreamExt as _;
use kuchiki::traits::TendrilSink as _;
use notify::{immediate_watcher, RecommendedWatcher, Watcher};
use tokio::{fs, sync::mpsc::unbounded_channel};
use tracing::{debug, info};

pub async fn build_continuously(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();
    let source_dir_abs = source_dir.canonicalize().with_context(|| {
        format!("Failed to canonicalize path `{}`", source_dir.display())
    })?;

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

        let trigger = match watch::Trigger::new(event) {
            Some(trigger) => trigger,
            None => continue,
        };

        let mut paths = String::new();

        let num_paths = trigger.paths.len();
        for (i, path) in trigger.paths.into_iter().enumerate() {
            // If we can't strip the prefix, just leave the path as-is.
            let path = path.strip_prefix(&source_dir_abs).unwrap_or(&path);

            write!(paths, "{}", path.display())?;
            if i < num_paths - 1 {
                write!(paths, ", ")?;
            }
        }

        info!("Building sites. Trigger: {} {}", trigger.kind, paths);

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
    let mut entries = WalkDir::new(&source_dir);

    while let Some(entry) = entries.next().await {
        let source = entry?.path();
        let output = output_dir.join(source.strip_prefix(&source_dir)?);

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
