mod transform;
mod walk;
mod watch;

pub use self::transform::Transform;

use std::path::Path;

use anyhow::{bail, Context as _};
use futures::StreamExt as _;
use html5ever::{tokenizer::TokenizerOpts, tree_builder::TreeBuilderOpts};
use kuchiki::{traits::TendrilSink as _, ParseOpts};
use tokio::{fs, sync::mpsc::unbounded_channel};
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
    build_html(&source_dir, &output_dir, transform)
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
    transform: &mut impl Transform,
) -> anyhow::Result<()> {
    let source_dir = source_dir.join("html");
    let output_dir = output_dir.to_path_buf();

    let mut entries = walk_dir(source_dir, output_dir);
    while let Some(entry) = entries.next().await {
        let (source, output) = entry?;

        if source.is_dir() {
            continue;
        }

        let (tx, mut rx) = unbounded_channel();
        let options = ParseOpts {
            tokenizer: TokenizerOpts {
                exact_errors: true,
                ..TokenizerOpts::default()
            },
            tree_builder: TreeBuilderOpts {
                exact_errors: true,
                ..TreeBuilderOpts::default()
            },
            on_parse_error: Some(Box::new(move |error| {
                // This method returns an error, if the received has been
                // closed. This shouldn't happen unless there's a bug, in which
                // case crashing this thread probably isn't the worst idea.
                tx.send(error).unwrap();
            })),
            ..ParseOpts::default()
        };

        let mut document = kuchiki::parse_html_with_options(options)
            .from_utf8()
            .from_file(&source)
            .with_context(|| {
                format!("Failed to parse HTML file `{}`", source.display())
            })?;

        if let Some(error) = rx.recv().await {
            // TASK: This will abort the whole process, but it should only abort
            //       this build run. It should be handle somewhere up the call
            //       chain.
            bail!("Error parsing `{}`: {}", source.display(), error);
        }

        transform.transform(&source, &mut document);

        // TASK: Transform document. Probably best to keep the transformations
        //       themselves out of this function and accept a closure that does
        //       them as an argument.

        document.serialize_to_file(output)?;
    }

    Ok(())
}
