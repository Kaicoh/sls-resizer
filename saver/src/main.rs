use failure::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::info;
use s3_service::ResizerInput;

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

fn handler(e: ResizerInput, _: Context) -> Result<(), HandlerError> {
    info!("Event: {:?}", e);
    Ok(())
}
