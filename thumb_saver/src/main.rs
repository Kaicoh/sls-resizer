use aws_lambda_events::event::s3::S3Event;
use dynamodb_service::Client;
use failure::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

fn handler(e: S3Event, _: Context) -> Result<(), HandlerError> {
    if e.records.is_empty() {
        bail!("Empty records");
    }

    let record = e.records[0].clone();
    let bucket = record.s3.bucket.name.unwrap();
    let key = record.s3.object.key.unwrap();

    Client::new().put(&bucket, &key, true).unwrap();

    Ok(())
}
