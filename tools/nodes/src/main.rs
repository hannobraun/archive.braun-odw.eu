mod secrets;

use std::process::Command;

fn main() -> anyhow::Result<()> {
    let name = "nodes-test";

    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(name)
        .arg("-f")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("nodes/")
        .status()?;

    println!("\n{}\n\n", status);

    let status = Command::new("docker").arg("run").arg(name).status()?;
    println!("\n{}", status);

    Ok(())
}
