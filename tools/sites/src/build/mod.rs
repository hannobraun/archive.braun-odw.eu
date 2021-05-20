mod html;
mod static_files;
mod transform;
mod walk;
mod watch;

pub use self::transform::Transform;

use std::path::Path;

use anyhow::Context as _;
use thiserror::Error;
use tokio::fs;
use tracing::{error, info};

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
        match build(source_dir, &output_dir, transform).await {
            Err(Error::ParseHtml(err)) => error!("{}", err),
            result => result?,
        }
    }

    Ok(())
}

pub async fn build(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    transform: &mut impl Transform,
) -> Result<(), Error> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir: {}", output_dir.display())
    })?;
    static_files::copy(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir: {}",
                output_dir.display()
            )
        })?;
    match html::process(&source_dir, &output_dir, transform).await {
        Err(html::Error::Parse(err)) => return Err(err)?,
        result => result.with_context(|| {
            format!(
                "Failed to process HTML files for `{}`",
                source_dir.display()
            )
        })?,
    }

    Ok(())
}

async fn prepare_output_dir(path: &Path) -> anyhow::Result<()> {
    if path.exists() {
        fs::remove_dir_all(path).await?;
    }
    fs::create_dir_all(path).await?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error parsing HTML")]
    ParseHtml(#[from] html::ParseError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
