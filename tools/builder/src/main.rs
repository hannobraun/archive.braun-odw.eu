use ssg::{
    args::Args,
    build::{build_once, html, watch::Watcher, Error},
    serve::serve_sites,
};
use tokio::fs;
use tracing::{debug, error, info};

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

async fn build_continuously(args: Args) -> anyhow::Result<()> {
    // Build at least once, before waiting for events.
    info!("Building sites.");
    build_all(args.clone()).await?;

    let mut watcher = Watcher::new(&args.source)?;
    while let Some(trigger) = watcher.watch().await? {
        info!("Building sites. Trigger: {}", trigger);
        match build_all(args.clone()).await {
            Err(Error::ParseSass(err)) => error!("{}", err),
            result => result?,
        }
    }

    Ok(())
}

async fn build_all(args: Args) -> Result<(), Error> {
    let mut entries = fs::read_dir(&args.source).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        debug!("Building `{}`", path.display());

        if path.is_file() {
            return Err(Error::InvalidSite(path));
        }

        let output_dir = args.target.join(path.file_name().unwrap());
        build_once(path, output_dir, Some(html::html(args.dev))).await?;
    }

    Ok(())
}
