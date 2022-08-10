use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::Args;

use zeebe_client::{api::CancelProcessInstanceRequest, ZeebeClient};

#[derive(Args)]
pub struct CancelProcessInstanceArgs {
    process_instance_key: i64,
}

impl From<&CancelProcessInstanceArgs> for CancelProcessInstanceRequest {
    fn from(args: &CancelProcessInstanceArgs) -> Self {
        CancelProcessInstanceRequest {
            process_instance_key: args.process_instance_key,
        }
    }
}

pub async fn handle_command(
    client: &mut ZeebeClient,
    args: &CancelProcessInstanceArgs,
) -> Result<Box<dyn Debug>> {
    let request: CancelProcessInstanceRequest = args.into();
    Ok(Box::new(
        client.cancel_process_instance(request).await?.into_inner(),
    ))
}
