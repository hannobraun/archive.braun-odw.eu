use std::path::Path;

pub async fn serve_sites(output_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    // TASK: Add logging.

    let sites_dir = output_dir.as_ref().join("sites");

    let filter = warp::filters::fs::dir(sites_dir);
    warp::serve(filter).run(([127, 0, 0, 1], 34480)).await;

    Ok(())
}
