use std::process::Command;

fn main() -> anyhow::Result<()> {
    let output = Command::new("docker")
        .arg("build")
        .arg("-f")
        .arg("tools/nodes/docker/Dockerfile")
        .arg("nodes/")
        .output()?;

    println!("{:?}", output);

    Ok(())
}
