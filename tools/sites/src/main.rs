mod args;
mod build;
mod serve;

use self::{
    args::Args,
    build::{build, build_continuously},
    serve::serve_sites,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let source_dir = "sites";
    let output_dir = "output";

    if args.dev {
        let build = build_continuously(source_dir, output_dir, args.dev);
        let serve = serve_sites(output_dir);
        tokio::try_join!(build, serve)?;
    } else {
        build(source_dir, output_dir, args.dev).await?;
    }

    Ok(())
}
