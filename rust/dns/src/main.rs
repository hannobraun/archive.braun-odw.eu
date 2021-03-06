mod domain;
mod secrets;

use self::{domain::Domain, secrets::Secrets};

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    let domain = Domain::load()?;

    println!("API token: {}", secrets.dns.api_token);
    println!("Domain ID: {}", domain.id);

    Ok(())
}
