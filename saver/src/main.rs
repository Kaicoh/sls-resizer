use failure::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::Deserialize;
use log::info;

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
    info!("Event: {:?}", e);
    Ok(())
}
