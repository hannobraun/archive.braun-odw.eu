use std::process::Command;

fn main() -> anyhow::Result<()> {
    let status = Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg("nodes-test")
        .arg("-f")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("nodes/")
        .status()?;

    println!("\n{}", status);

    Ok(())
}
