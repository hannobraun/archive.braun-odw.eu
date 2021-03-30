mod args;
mod build;

use std::path::Path;

use clap::Clap as _;

use self::{args::Args, build::build};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source_dir = Path::new("sites");
    let output_dir = Path::new("output");

    build(source_dir, output_dir).await?;
    if args.serve {
        serve_sites(output_dir).await?;
    }

    Ok(())
}

async fn serve_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let sites_dir = output_dir.as_ref().join("sites");

    let filter = warp::filters::fs::dir(sites_dir);
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
