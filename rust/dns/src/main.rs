mod secrets;

use self::secrets::Secrets;

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    println!("API token: {}", secrets.dns.api_token);
    Ok(())
}
