use anyhow::{Context, Error};
use dotenv::dotenv;
use infrastructure::inegi_client::InegiClient;
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    dotenv().ok();
    let api_key =
        env::var("INEGI_API_KEY").with_context(|| "INEG_API_KEY env variable not defined")?;
    let max_retries = 5;
    let connection_timeout_seconds: u64 = 10;
    let client = InegiClient::new(api_key, max_retries, connection_timeout_seconds);
    let data = client.get_data_from_mgee_service("01".to_string()).await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {}
