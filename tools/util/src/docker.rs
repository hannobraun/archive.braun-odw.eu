use std::process::{Command, ExitStatus};

use anyhow::bail;

pub fn build_and_run(name: &str, file: &str, arg: &str) -> anyhow::Result<()> {
    build(name, file, arg)?;

    let status = run(name)?;
    println!("\n{}", status);

    Ok(())
}

pub fn build(name: &str, file: &str, arg: &str) -> anyhow::Result<()> {
    let status = Command::new("docker")
        .arg("build")
        .arg("--tag")
        .arg(name)
        .arg("--file")
        .arg(file)
        .arg("--build-arg")
        .arg(arg)
        .arg("nodes/")
        .status()?;

    if !status.success() {
        bail!("`docker build` invocation failed: {}", status);
    }

    Ok(())
}

pub fn run(name: &str) -> anyhow::Result<ExitStatus> {
    let status = Command::new("docker").arg("run").arg(name).status()?;

    if !status.success() {
        bail!("`docker run` invocation failed: {}", status);
    }

    // TASK: Don't return status.
    Ok(status)
}
