use failure::Error;
use image::GenericImageView;
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
    let new_key = key.replace("uploads/", "thumbnails/crop_");

    let client = Client::new();
    client
        .get(&bucket, &key)
        .map(crop_and_grayscale)
        .map(|buffer| client.put(&bucket, &new_key, buffer))
        .unwrap();

    Ok(())
}

fn crop_and_grayscale(buffer: Vec<u8>) -> Vec<u8> {
    let mut out_buffer: Vec<u8> = Vec::new();
    let uploaded_image = image::load_from_memory(&buffer).unwrap();
    let (width, height) = (uploaded_image.width(), uploaded_image.height());
    let (x, y, width, height) = if width > height {
        ((width - height) / 2, 0, height, height)
    } else {
        (0, (height - width) / 2, width, width)
    };

    uploaded_image
        .crop_imm(x, y, width, height)
        .grayscale()
        .write_to(&mut out_buffer, ImageFormat::Jpeg)
        .unwrap();

    out_buffer
}
