use std::time::Duration;

use crate::inegi_api::{api_response::ApiResponse, mgee_responses::MgeeResponse};
use anyhow::{Context, Error};
use reqwest::Client;
use tokio::time::sleep;

#[derive(Debug)]
pub struct InegiClient {
    api_key: String,
    max_retries: u32,
    connection_timeout_seconds: u64,
}

impl InegiClient {
    pub fn new(api_key: String, max_retries: u32, connection_timeout_seconds: u64) -> InegiClient {
        InegiClient {
            api_key,
            max_retries,
            connection_timeout_seconds,
        }
    }

    pub async fn get_data_from_mgee_service(
        &self,
        entidad_id: String,
    ) -> Result<MgeeResponse, Error> {
        let url = format!("https://gaia.inegi.org.mx/wscatgeo/v2/mgee/{}/", entidad_id);
        let match_data = Self::api_request(self, &url).await?;
        match match_data {
            ApiResponse::MgeeSuccesResponse {
                datos,
                metadatos,
                num_reg,
            } => Ok(MgeeResponse::new(datos, metadatos, num_reg)),
            ApiResponse::MgeeErrorResponse { result, mensaje } => Err(anyhow::anyhow!(format!(
                "Error {result} fetching data from {url}: {mensaje}"
            ))),
            _ => Err(anyhow::anyhow!(format!("Unexpected response from {url}"))),
        }
    }

    async fn api_request(&self, url: &String) -> Result<ApiResponse, Error> {
        let mut retries = 0;
        let mut backoff = Duration::from_millis(500);

        let client = Client::builder()
            .connect_timeout(Duration::from_secs(self.connection_timeout_seconds))
            .build()
            .with_context(|| "Failed to build HTTP Client")?;
        loop {
            match client
                .get(url)
                .send()
                .await
                .with_context(|| format!("Failed to send GET request to {}", &url))?
                .json::<ApiResponse>()
                .await
            {
                Ok(body) => return Ok(body),
                Err(err) if retries < self.max_retries => {
                    retries += 1;
                    tracing::warn!(
                        "Request to {url} failed (attempt {retries}): {err} retrying ..."
                    );
                    sleep(backoff).await;
                    backoff *= 2;
                    continue;
                }
                Err(err) => {
                    return Err(err).with_context(|| {
                        format!("Failed to parse or get JSON from {url}, after {retries} retries",)
                    });
                }
            }
        }
    }
}
