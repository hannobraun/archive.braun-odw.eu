use std::process::Command;

use anyhow::bail;

pub fn build_and_run(
    name: &str,
    file: &str,
    args: &[&str],
    path: &str,
) -> anyhow::Result<()> {
    build(name, file, args, path)?;
    run(name)?;

    Ok(())
}

pub fn build(
    name: &str,
    file: &str,
    args: &[&str],
    path: &str,
) -> anyhow::Result<()> {
    let mut command = Command::new("docker");

    command
        .arg("build")
        .arg("--tag")
        .arg(name)
        .arg("--file")
        .arg(file);

    for arg in args {
        command.arg("--build-arg").arg(arg);
    }

    let status = command.arg(path).status()?;

    if !status.success() {
        bail!("`docker build` failed: {}", status);
    }

    Ok(())
}

pub fn run(name: &str) -> anyhow::Result<()> {
    let status = Command::new("docker").arg("run").arg(name).status()?;

    if !status.success() {
        bail!("`docker run` failed: {}", status);
    }

    Ok(())
}
