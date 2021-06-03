use ssg::{args::Args, build::{build_once, html::html}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    build_once(&args.source, &args.target, Some(html(args.dev))).await?;

    Ok(())
}
