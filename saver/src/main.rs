use failure::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use s3_service::ResizerInput;
use dynamodb_service::Client;

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

fn handler(e: ResizerInput, _: Context) -> Result<(), HandlerError> {
    let bucket = e.bucket;
    let key = e.key;

    Client::new().put(&bucket, &key, false).unwrap();

    Ok(())
}
