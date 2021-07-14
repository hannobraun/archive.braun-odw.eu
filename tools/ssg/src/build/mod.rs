pub mod sass;
pub mod static_files;
pub mod walk;
pub mod watch;

use std::path::{Path, PathBuf};

use anyhow::Context as _;
use thiserror::Error;
use tokio::{fs, io};
use tracing::{error, info};

use crate::html::{self, model::Element};

pub async fn build_once(
    source_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    html: Option<Element>,
) -> Result<(), Error> {
    let source_dir = source_dir.as_ref();
    let output_dir = output_dir.as_ref();

    info!(
        "Building `{}` (output: `{}`)",
        source_dir.display(),
        output_dir.display()
    );

    prepare_output_dir(&output_dir).await.with_context(|| {
        format!("Failed to prepare output dir (`{}`)", output_dir.display())
    })?;

    static_files::copy(&source_dir, &output_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to copy sites to output dir (`{}`)",
                output_dir.display()
            )
        })?;
    if let Some(html) = html {
        html::build(output_dir, html).await.with_context(|| {
            format!("Failed to build HTML to `{}`", output_dir.display())
        })?;
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
    #[error("I/O error ({path:?})")]
    Io {
        source: io::Error,
        path: Option<PathBuf>,
    },

    #[error("Found file in `sites/` directory")]
    InvalidSite(PathBuf),

    #[error("Error parsing SASS")]
    ParseSass(#[from] rsass::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
