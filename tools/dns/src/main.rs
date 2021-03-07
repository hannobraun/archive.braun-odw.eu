mod domain;
mod secrets;
mod util;

use self::{domain::Domain, secrets::Secrets};

fn main() -> anyhow::Result<()> {
    let secrets = Secrets::load()?;
    let domain = Domain::load("braun-odw.eu")?;

    println!("API token: {}", secrets.dns.api_token);
    println!("Domain ID: {}", domain.id);

    // TASK: Decide on approach to accessing Hetzner DNS REST API. restson looks
    //       promising, but I'm getting weird linker errors when compiling it,
    //       from openssl-sys.

    // TASK: Validate zone file.
    // TASK: Upload zone file.

    Ok(())
}
