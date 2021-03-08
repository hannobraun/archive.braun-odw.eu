mod domain;
mod secrets;
mod util;

use self::{domain::Domain, secrets::Secrets};

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    let domain = Domain::load("braun-odw.eu")?;

    println!("API token: {}", secrets.dns.api_token);
    println!("Domain ID: {}", domain.id);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://dns.hetzner.com/api/v1/zones/file/validate")
        .send()?;
    println!("{} {}", response.status(), response.text()?);

    // TASK: Validate zone file.
    // TASK: Upload zone file.

    Ok(())
}
