use std::process::Command;

use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    println!("Private key: {}", secrets.nodes["reineke"].ssh_key);

    let name = "nodes-test";

    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(name)
        .arg("-f")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("nodes/")
        .status()?;
    // TASK: Abort, if command was unsuccessful.

    println!("\n{}\n\n", status);

    let status = Command::new("docker").arg("run").arg(name).status()?;
    println!("\n{}", status);

    Ok(())
}
