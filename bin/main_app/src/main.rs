use anyhow::{Context, Error};
use dotenv::dotenv;
use infrastructure::{inegi_api::inegi_client::InegiClient, minio::minio_client::MinioClient};
use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    dotenv().ok();
    let api_key =
        env::var("INEGI_API_KEY").with_context(|| "INEG_API_KEY env variable not defined")?;
    let minio_endpoint =
        env::var("MINIO_ENDPOINT").with_context(|| "MINIO_ENDPOINT env variable not defined")?;

    let max_retries = 5;
    let connection_timeout_seconds: u64 = 10;
    let inegi_client = InegiClient::new(api_key, max_retries, connection_timeout_seconds);
    let data = inegi_client
        .get_data_from_mgee_service("01".to_string())
        .await?;
    let minio_client = MinioClient::new("minioadmin", "minioadmin", &minio_endpoint)?;
    minio_client.create_bucket("test-bucket").await?;
    minio_client
        .upload_to_minio(
            "test-bucket",
            "test-object.json",
            serde_json::to_vec_pretty(&data).unwrap(),
            "application/json",
        )
        .await?;
    println!(
        "{} uploaded to bucket",
        serde_json::to_string_pretty(&data).unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod tests {}
