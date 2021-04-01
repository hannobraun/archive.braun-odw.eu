use std::path::Path;

use warp::Filter as _;

pub async fn serve_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let sites_dir = output_dir.as_ref().join("sites/static");

    let filter =
        warp::filters::fs::dir(sites_dir).with(warp::filters::log::log(""));
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
