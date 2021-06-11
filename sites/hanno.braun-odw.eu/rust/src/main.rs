use ssg::{args::Args, build::build_once};

mod data;
mod html;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let ongoing_work = data::OngoingWork::load();
    let side_projects = data::SideProjects::load();

    let html = html::build(args.dev, ongoing_work, side_projects);
    build_once(&args.source, &args.target, Some(html)).await?;

    Ok(())
}
