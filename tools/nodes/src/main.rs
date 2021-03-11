use std::process::Command;

fn main() -> anyhow::Result<()> {
    let status = Command::new("docker")
        .arg("build")
        .arg("-f")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("nodes/")
        .status()?;

    println!("{:?}", status);

    Ok(())
}
