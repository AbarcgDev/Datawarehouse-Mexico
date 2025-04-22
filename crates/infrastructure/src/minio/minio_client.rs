use std::io::Cursor;

use anyhow::{Context, Result};
use minio::s3::{
    args::{BucketArgs, MakeBucketArgs, PutObjectArgs},
    client::{Client, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};

pub struct MinioClient {
    client: Client,
}

impl MinioClient {
    pub fn new(minio_user: &str, minio_secret: &str, endpoint_url: &str) -> Result<MinioClient> {
        let base_url = endpoint_url
            .parse::<BaseUrl>()
            .with_context(|| "Failed to parse MinIO URL")?;
        let provider = StaticProvider::new(minio_user, minio_secret, None);
        let client = ClientBuilder::new(base_url.clone())
            .provider(Some(Box::new(provider)))
            .build()
            .with_context(|| "Failed to create MinIO Client")?;

        Ok(Self { client })
    }

    pub async fn upload_to_minio(
        &self,
        bucket_name: &str,
        object_name: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> anyhow::Result<()> {
        let mut object_content = Cursor::new(&content);
        let mut put_object_args = PutObjectArgs::new(
            bucket_name,
            object_name,
            &mut object_content,
            Some(content.len()),
            None,
        )?;
        put_object_args.content_type = content_type;
        self.client.put_object(&mut put_object_args).await?;
        Ok(())
    }

    pub async fn create_bucket(&self, bucket_name: &str) -> anyhow::Result<()> {
        tracing::info!("Trying to create bucket with name {}", bucket_name);
        let bucket_args = BucketArgs::new(bucket_name)?;
        let response = self.client.bucket_exists(&bucket_args).await?;
        if !response {
            let make_args = MakeBucketArgs::new(bucket_name)?;
            self.client
                .make_bucket(&make_args)
                .await
                .with_context(|| format!("Error creating bucket with name {}", bucket_name))?;
        }
        Ok(())
    }
}
