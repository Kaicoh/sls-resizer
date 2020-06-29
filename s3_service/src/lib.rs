use failure::Error;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResizerInput {
    pub bucket: String,
    pub key: String,
}

pub struct Client(S3Client);

impl Client {
    pub fn new() -> Self {
        Self(S3Client::new(Region::ApNortheast1))
    }

    #[tokio::main]
    pub async fn get(&self, bucket: &str, key: &str) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = Vec::new();
        let input = GetObjectRequest {
            bucket: bucket.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };
        let data = self.0.get_object(input).await.unwrap();
        data.body
            .unwrap()
            .into_async_read()
            .read_to_end(&mut buffer)
            .await
            .unwrap();
        Ok(buffer)
    }

    #[tokio::main]
    pub async fn put(&self, bucket: &str, key: &str, buffer: Vec<u8>) {
        let input = PutObjectRequest {
            body: Some(buffer.into()),
            bucket: bucket.to_owned(),
            key: key.to_owned(),
            content_type: Some("image/jpg".to_owned()),
            ..Default::default()
        };
        self.0.put_object(input).await.unwrap();
    }
}
