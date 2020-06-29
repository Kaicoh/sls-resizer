use failure::Error;
use image::ImageFormat;
use lambda_runtime::{error::HandlerError, lambda, Context};
use s3_service::{Client, ResizerInput};

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

fn resize(buffer: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    image::load_from_memory(&buffer)
        .unwrap()
        .thumbnail(width, height)
        .write_to(&mut out_buffer, ImageFormat::Jpeg)
        .unwrap();
    out_buffer
}
