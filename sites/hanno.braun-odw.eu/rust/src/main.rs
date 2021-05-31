use ssg::build::build_once;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let source_dir = "sites";
    let output_dir = "output";

    build_once(source_dir, output_dir, None).await?;

    Ok(())
}
