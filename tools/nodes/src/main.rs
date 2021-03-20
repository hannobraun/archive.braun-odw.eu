use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;

    let name = "nodes-test";
    util::docker::build_and_run(
        name,
        "tools/nodes/docker/Dockerfile",
        &format!("SSH_KEY={}", secrets.nodes["reineke"].ssh_key),
    )?;

    Ok(())
}
