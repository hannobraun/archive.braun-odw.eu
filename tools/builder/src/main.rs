use ssg::{
    args::Args,
    build::{build_all, build_continuously},
    serve::serve_sites,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    if args.dev {
        let build = build_continuously(args.clone());
        let serve = serve_sites(args.target);
        tokio::try_join!(build, serve)?;
    } else {
        build_all(args).await?;
    }

    Ok(())
}
