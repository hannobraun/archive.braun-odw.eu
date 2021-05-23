mod html;
mod sass;
mod static_files;
mod transform;
mod walk;
mod watch;

pub use self::transform::Transform;

use std::path::{Path, PathBuf};

use anyhow::Context as _;
use thiserror::Error;
use tokio::{fs, io};
use tracing::{error, info};

// TASK: Handle SCSS compile errors, don't let them end the application.
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

    let mut entries = fs::read_dir(source_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            return Err(Error::InvalidSite(path));
        }

        let output_dir = output_dir.join(path.file_name().unwrap());
        build_site(path, output_dir, transform).await?;
    }

    Ok(())
}

async fn build_site(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    transform: &mut impl Transform,
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
    match html::process(&source_dir, &output_dir, transform).await {
        Err(html::Error::Parse(err)) => return Err(err)?,
        result => result.with_context(|| {
            format!(
                "Failed to process HTML files for `{}`",
                source_dir.display()
            )
        })?,
    }
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

    #[error("Error parsing HTML")]
    ParseHtml(#[from] html::ParseError),

    #[error("Error parsing SASS")]
    ParseSass(#[from] rsass::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
