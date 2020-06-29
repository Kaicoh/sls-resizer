use aws_lambda_events::event::s3::S3Event;
use failure::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::Region;
use rusoto_stepfunctions::{StepFunctionsClient, StepFunctions, ListStateMachinesInput, StartExecutionInput};
use simple_error::bail;
use serde_derive::Serialize;

#[derive(Serialize)]
struct ResizerInput {
    bucket: String,
    key: String,
}

fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(handler);
    Ok(())
}

#[tokio::main]
async fn handler(e: S3Event, _: Context) -> Result<(), HandlerError> {
    if e.records.is_empty() {
        bail!("Empty records");
    }

    let record = e.records[0].clone();
    let bucket = record.s3.bucket.name.unwrap();
    let key = record.s3.object.key.unwrap();

    let client = StepFunctionsClient::new(Region::ApNortheast1);
    let list_input = ListStateMachinesInput { ..Default::default() };
    let state_machine_list = client.list_state_machines(list_input).await.unwrap();
    let state_machine = match state_machine_list
        .state_machines
        .iter()
        .find(|item| item.name.starts_with("ImageProcessing")) {
            Some(sm) => sm,
            None => {
                bail!("State Machine Not Found");
        }
    };

    let params = ResizerInput {
        bucket: bucket.to_owned(),
        key: key.to_owned(),
    };
    let exec_input = StartExecutionInput {
        state_machine_arn: state_machine.state_machine_arn.clone(),
        name: None,
        input: Some(serde_json::to_string(&params)?),
    };
    client.start_execution(exec_input).await.unwrap();

    Ok(())
}
