pub mod html;
pub mod sass;
pub mod static_files;
pub mod walk;
pub mod watch;

use std::path::{Path, PathBuf};

use anyhow::Context as _;
use thiserror::Error;
use tokio::{fs, io};
use tracing::{error, info};

use crate::args::Args;

use self::html::model::Element;

pub async fn build_continuously(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    args: Args,
) -> anyhow::Result<()> {
    let source_dir = source_dir.as_ref();

    // Build at least once, before waiting for events.
    info!("Building sites.");
    build_all(source_dir, &output_dir, args.dev).await?;

    let mut watcher = watch::Watcher::new(source_dir)?;
    while let Some(trigger) = watcher.watch().await? {
        info!("Building sites. Trigger: {}", trigger);
        match build_all(source_dir, &output_dir, args.dev).await {
            Err(Error::ParseSass(err)) => error!("{}", err),
            result => result?,
        }
    }

    Ok(())
}

pub async fn build_all(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    dev: bool,
) -> Result<(), Error> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir: {}", output_dir.display())
    })?;

    let mut entries = fs::read_dir(source_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            return Err(Error::InvalidSite(path));
        }

        let output_dir = output_dir.join(path.file_name().unwrap());
        build_once(path, output_dir, html::html(dev)).await?;
    }

    Ok(())
}

async fn build_once(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    html: Element,
) -> Result<(), Error> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    static_files::copy(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir: {}",
                output_dir.display()
            )
        })?;
    html::build(output_dir, html).await?;
    match sass::compile(&source_dir, &output_dir).await {
        Err(sass::Error::Parse(err)) => return Err(err)?,
        result => result.with_context(|| {
            format!(
                "Failed to compile SASS files for `{}`",
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
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("Found file in `sites/` directory")]
    InvalidSite(PathBuf),

    #[error("Error parsing SASS")]
    ParseSass(#[from] rsass::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
