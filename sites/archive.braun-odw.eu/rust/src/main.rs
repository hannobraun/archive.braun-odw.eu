use ssg::{args::Args, build::build_once};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    build_once(&args.source, &args.target, None).await?;

    Ok(())
}
