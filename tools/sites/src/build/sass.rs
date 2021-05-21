use std::path::Path;

use futures::StreamExt as _;
use rsass::{compile_scss_path, output::Format};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::build::walk::walk_dir;

pub async fn compile(
    source_dir: &Path,
    output_dir: &Path,
) -> anyhow::Result<()> {
    let source_dir = source_dir.join("sass");
    let output_dir = output_dir.to_path_buf();

    if !source_dir.exists() {
        return Ok(());
    }

    let mut entries = walk_dir(source_dir, output_dir);
    while let Some(entry) = entries.next().await {
        let (source, output) = entry?;

        let css = compile_scss_path(&source, Format::default())?;

        let mut output = File::create(output).await?;
        output.write_all(&css).await?;
    }

    Ok(())
}
