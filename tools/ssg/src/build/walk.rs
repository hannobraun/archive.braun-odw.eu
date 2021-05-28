use std::path::PathBuf;

use async_walkdir::WalkDir;
use futures::{Stream, StreamExt};

pub fn walk_dir(
    source_dir: PathBuf,
    output_dir: PathBuf,
) -> impl Stream<Item = anyhow::Result<(PathBuf, PathBuf)>> {
    WalkDir::new(&source_dir).map(move |entry| {
        let source = entry?.path();
        let output = output_dir.join(source.strip_prefix(&source_dir)?);

        Ok((source, output))
    })
}
