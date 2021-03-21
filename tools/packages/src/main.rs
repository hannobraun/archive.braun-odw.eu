fn main() -> anyhow::Result<()> {
    util::docker::build_and_run(
        "packages-test",
        "tools/packages/Dockerfile",
        &[],
        "packages/",
    )?;

    // TASK: Deploy `sites`.

    Ok(())
}
