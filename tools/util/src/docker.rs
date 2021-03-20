use std::process::{Command, ExitStatus};

pub fn build(name: &str, file: &str, arg: &str) -> anyhow::Result<ExitStatus> {
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

    // TASK: Abort, if command was unsuccessful.

    Ok(status)
}
