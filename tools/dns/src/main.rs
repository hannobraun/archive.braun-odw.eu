mod api;
mod domain;
mod secrets;
mod util;

use self::{api::validate_zone, domain::Domain, secrets::Secrets};

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    let domain = Domain::load("braun-odw.eu")?;

    println!("Domain ID: {}", domain.id);

    validate_zone(secrets.dns.api_token, domain.zone)?;

    // TASK: Upload zone file.

    Ok(())
}
