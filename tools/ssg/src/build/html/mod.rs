#[macro_use]
pub mod front_macro;
pub mod model;

use std::{
    io::{self, Write},
    path::Path,
};

use tokio::{fs::File, io::AsyncWriteExt as _};
use tracing::info;

use self::model::Element;

pub async fn build(
    output_dir: impl AsRef<Path>,
    html: Element,
) -> io::Result<()> {
    let output = output_dir.as_ref().join("index.html");
    let mut target = Vec::new();

    info!("Building HTML document `{}`", output.display());

    writeln!(target, "<!DOCTYPE html>")?;
    html.write_to(&mut target)?;

    File::create(output).await?.write_all(&target).await?;

    Ok(())
}
