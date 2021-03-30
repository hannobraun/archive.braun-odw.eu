mod args;
mod build;

use std::path::Path;

use build::build_continuously;
use clap::Clap as _;

use self::{args::Args, build::build};

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

async fn serve_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // TASK: Add logging.

    let sites_dir = output_dir.as_ref().join("sites");

    let filter = warp::filters::fs::dir(sites_dir);
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
