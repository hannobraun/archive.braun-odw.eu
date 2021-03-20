use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;

    let name = "nodes-test";

    let status = util::docker::build(
        "nodes-test",
        "tools/nodes/docker/Dockerfile",
        &format!("SSH_KEY={}", secrets.nodes["reineke"].ssh_key),
    )?;

    println!("\n{}\n\n", status);

    let status = util::docker::run(name)?;
    println!("\n{}", status);

    Ok(())
}
