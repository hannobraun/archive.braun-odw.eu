use anyhow::bail;
use reqwest::StatusCode;

pub struct Api {
    api_token: String,
    client: reqwest::blocking::Client,
}

impl Api {
    pub fn new(api_token: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Self { api_token, client }
    }

    pub fn validate_zone(&self, zone: String) -> anyhow::Result<()> {
        let response = self
            .client
            .post("https://dns.hetzner.com/api/v1/zones/file/validate")
            .header("Auth-API-Token", &self.api_token)
            .header("Content-Type", "text/plain")
            .body(zone)
            .send()?;

        if response.status() != StatusCode::OK {
            bail!(
                "Zone not validated: {} {}",
                response.status(),
                response.text()?
            );
        }

        Ok(())
    }

    pub fn upload_zone(&self, id: &str, zone: String) -> anyhow::Result<()> {
        let path =
            format!("https://dns.hetzner.com/api/v1/zones/{}/import", id);

        let response = self
            .client
            .post(&path)
            .header("Auth-API-Token", &self.api_token)
            .header("Content-Type", "text/plain")
            .body(zone)
            .send()?;

        if response.status() != StatusCode::OK {
            bail!(
                "Zone not uploaded: {} {}",
                response.status(),
                response.text()?
            );
        }

        Ok(())
    }
}
