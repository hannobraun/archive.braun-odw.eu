use ssg::{args::Args, build::{build_once, html::html}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let html = Some(html(args.dev));
    build_once(&args.source, &args.target, html).await?;

    Ok(())
}
