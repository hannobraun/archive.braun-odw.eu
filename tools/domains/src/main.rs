mod api;
mod domain;
mod secrets;

use self::{api::Api, domain::Domain, secrets::Secrets};

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    let domain = Domain::load("braun-odw.eu")?;

    println!("Domain ID: {}", domain.id);

    let api = Api::new(secrets.domains.api_token);
    api.validate_zone(domain.zone.clone())?;
    api.upload_zone(&domain.id, domain.zone)?;

    Ok(())
}
