use std::{io, path::Path};

use anyhow::Context as _;
use futures::StreamExt as _;
use html5ever::{
    tendril::TendrilSink as _, tokenizer::TokenizerOpts,
    tree_builder::TreeBuilderOpts,
};
use kuchiki::ParseOpts;
use thiserror::Error;
use tokio::sync::mpsc::unbounded_channel;

use super::{walk::walk_dir, Transform};

pub async fn process(
    source_dir: &Path,
    output_dir: &Path,
    transform: &mut impl Transform,
) -> Result<(), Error> {
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
            return Err(ParseError {
                file: source.to_string_lossy().into(),
                message: error.into(),
            })?;
        }

        transform.transform(&source, &mut document)?;

        document.serialize_to_file(output)?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error("Error transforming HTML")]
    Transform(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
#[error("Error parsing `{file}`: {message}")]
pub struct ParseError {
    file: String,
    message: String,
}
