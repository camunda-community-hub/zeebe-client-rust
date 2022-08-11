use async_trait::async_trait;
use color_eyre::eyre::Result;

use clap::Args;
use zeebe_client::{
    api::{CancelProcessInstanceRequest, CancelProcessInstanceResponse},
    ZeebeClient,
};

use crate::ExecuteZeebeCommand;

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

#[async_trait]
impl ExecuteZeebeCommand for CancelProcessInstanceArgs {
    type Output = CancelProcessInstanceResponse;
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .cancel_process_instance(CancelProcessInstanceRequest::from(&self))
            .await?
            .into_inner())
    }
}
