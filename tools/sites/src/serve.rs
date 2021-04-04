use std::path::PathBuf;

use warp::Filter as _;

pub async fn serve_sites(output_dir: impl Into<PathBuf>) -> anyhow::Result<()> {
    // TASK: Serve a list of links to all available sites at `/`.

    let filter = warp::filters::fs::dir(output_dir.into())
        .with(warp::filters::log::log(""));
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
