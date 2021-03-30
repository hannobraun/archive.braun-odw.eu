mod args;
mod build;

use std::path::Path;

use clap::Clap as _;

use self::args::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let output_dir = Path::new("output");

    // TASK: Re-build sites, if contents change.
    build::prepare_output_dir(output_dir)?;
    build::copy_sites(output_dir)?;

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
