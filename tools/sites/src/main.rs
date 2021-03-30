mod args;
mod build;
mod serve;

use std::path::Path;

use build::build_continuously;
use clap::Clap as _;

use self::{args::Args, build::build, serve::serve_sites};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source_dir = Path::new("sites");
    let output_dir = Path::new("output");

    if args.serve {
        let build = build_continuously(source_dir, output_dir);
        let serve = serve_sites(output_dir);
        tokio::try_join!(build, serve)?;
    } else {
        build(source_dir, output_dir).await?;
    }

    Ok(())
}
