use std::env;

use anyhow::Context as _;
use ssg::{
    args::Args,
    build::{watch::Watcher, Error},
    serve::serve_sites,
};
use tokio::{fs, process::Command};
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

// TASK: Only rebuild those websites where changes have been detected.
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
        fs::create_dir_all(&output_dir).await?;

        let source = path.canonicalize().with_context(|| {
            format!("Failed to canonicalize source path (`{}`)", path.display())
        })?;
        let target = output_dir.canonicalize().with_context(|| {
            format!(
                "Failed to canonicalize target path (`{}`)",
                output_dir.display()
            )
        })?;

        let current_dir = env::current_dir()?;
        env::set_current_dir(path.join("rust"))?;

        let mut command = Command::new("cargo");

        command
            .arg("run")
            .arg("--")
            .args(&["--source", source.to_str().unwrap()])
            .args(&["--target", target.to_str().unwrap()]);

        if args.dev {
            command.arg("--dev");
        }

        let status = command
            .status()
            .await
            .context("Failed to run site builder")?;

        env::set_current_dir(current_dir)?;

        if !status.success() {
            error!("Failed to execute site builder. Status code: {}", status);
        }
    }

    Ok(())
}
