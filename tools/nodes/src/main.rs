use std::process::Command;

use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;

    let name = "nodes-test";

    let status = Command::new("docker")
        .arg("build")
        .arg("--tag")
        .arg(name)
        .arg("--file")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("--build-arg")
        .arg(format!("SSH_KEY={}", secrets.nodes["reineke"].ssh_key))
        .arg("nodes/")
        .status()?;
    // TASK: Abort, if command was unsuccessful.

    println!("\n{}\n\n", status);

    let status = Command::new("docker").arg("run").arg(name).status()?;
    println!("\n{}", status);

    Ok(())
}
