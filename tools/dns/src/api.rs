pub fn validate_zone(api_token: String, zone: String) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://dns.hetzner.com/api/v1/zones/file/validate")
        .header("Auth-API-Token", api_token)
        .header("Content-Type", "text/plain")
        .body(zone)
        .send()?;

    println!("{} {}", response.status(), response.text()?);

    Ok(())
}
