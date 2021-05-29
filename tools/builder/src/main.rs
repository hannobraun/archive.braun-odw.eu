use ssg::{
    args::Args,
    build::{build_all, build_continuously},
    serve::serve_sites,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let source_dir = "sites";
    let output_dir = "output";

    if args.dev {
        let build = build_continuously(source_dir, output_dir, args);
        let serve = serve_sites(output_dir);
        tokio::try_join!(build, serve)?;
    } else {
        build_all(source_dir, output_dir, args.dev).await?;
    }

    Ok(())
}
