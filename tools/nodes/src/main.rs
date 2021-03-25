use std::{fs, path::Path};

use anyhow::Context;
use fs_extra::dir::CopyOptions;
use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;

    let docker_dir = "docker/";
    prepare_docker_dir(docker_dir).with_context(|| {
        format!("Failed to prepare docker directory `{}`", docker_dir)
    })?;

    util::docker::build_and_run(
        "nodes-test",
        "tools/nodes/Dockerfile",
        &[&format!("SSH_KEY={}", secrets.nodes["reineke"].ssh_key)],
        docker_dir,
    )?;

    Ok(())
}

fn prepare_docker_dir(path: impl AsRef<Path>) -> anyhow::Result<()> {
    if path.as_ref().exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    fs_extra::copy_items(&["nodes/"], &path, &CopyOptions::new())?;
    Ok(())
}
