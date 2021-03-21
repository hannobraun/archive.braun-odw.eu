// TASK: Package `packages/sites` as Nix package and deploy to Reineke:
//       - https://nixos.org/manual/nixpkgs/stable/#compiling-rust-applications-with-cargo
//       - https://nixos.wiki/wiki/Packaging/Tutorial

fn main() -> anyhow::Result<()> {
    util::docker::build_and_run(
        "packages-test",
        "tools/packages/Dockerfile",
        &[],
        "packages/",
    )?;

    Ok(())
}
