use failure::Error;
use image::ImageFormat;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct ResizerInput {
    bucket: String,
    key: String,
}

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

fn handler(e: ResizerInput, _: Context) -> Result<(), HandlerError> {
    let bucket = e.bucket;
    let key = e.key;
    let new_key = key.replace("uploads/", "thumbnails/");

    let client = Client::new();
    client
        .get(&bucket, &key)
        .map(|buffer| resize(buffer, 480, 270))
        .map(|buffer| client.put(&bucket, &new_key, buffer))
        .unwrap();

    Ok(())
}

struct Client(S3Client);

impl Client {
    fn new() -> Self {
        Self(S3Client::new(Region::ApNortheast1))
    }

    #[tokio::main]
    async fn get(&self, bucket: &str, key: &str) -> Result<Vec<u8>, Error> {
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
    async fn put(&self, bucket: &str, key: &str, buffer: Vec<u8>) {
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

fn resize(buffer: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    image::load_from_memory(&buffer)
        .unwrap()
        .thumbnail(width, height)
        .write_to(&mut out_buffer, ImageFormat::Jpeg)
        .unwrap();
    out_buffer
}