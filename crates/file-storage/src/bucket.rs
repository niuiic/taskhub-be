use anyhow::{Context, Result};
use s3::{creds::Credentials, Bucket, Region};

use crate::config::Minio;

pub fn get_bucket(config: &Minio) -> Result<Bucket> {
    Bucket::new(
        &config.bucket,
        Region::Custom {
            region: String::new(),
            endpoint: format!("{}:{}", &config.ip, &config.port),
        },
        Credentials {
            access_key: Some(config.access_key.clone()),
            secret_key: Some(config.secret_key.clone()),
            security_token: None,
            session_token: None,
            expiration: None,
        },
    )
    .context("failed to create minio bucket handle")
}
