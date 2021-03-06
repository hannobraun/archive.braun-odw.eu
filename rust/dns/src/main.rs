use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    let mut secrets = Vec::new();
    File::open("secrets.toml")?.read_to_end(&mut secrets)?;

    println!("{:?}", secrets);

    Ok(())
}
