use util::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;

    util::docker::build_and_run(
        "nodes-test",
        "tools/nodes/docker/Dockerfile",
        &format!("SSH_KEY={}", secrets.nodes["reineke"].ssh_key),
    )?;

    Ok(())
}
