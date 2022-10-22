use async_trait::async_trait;
use color_eyre::eyre::Result;

use crate::ExecuteZeebeCommand;
use clap::Args;

use zeebe_client::{
    api::{FailJobRequest, FailJobResponse},
    ZeebeClient,
};
#[derive(Args)]
pub(crate) struct FailJobArgs {
    // the unique job identifier, as obtained when activating the job
    #[arg(short, long)]
    job_key: i64,
    // the amount of retries the job should have left
    #[arg(short, long)]
    retries: i32,
    // an optional message describing why the job failed
    // this is particularly useful if a job runs out of retries and an incident is raised,
    // as it this message can help explain why an incident was raised
    #[arg(long, required = false, default_value = "")]
    error_message: String,
    // the back off timeout for the next retry
    #[arg(long, required = false, default_value_t = 0)]
    retry_back_off: i64,
    #[arg(long, required = false, default_value = "")]
    variables: String,
}

impl From<&FailJobArgs> for FailJobRequest {
    fn from(args: &FailJobArgs) -> Self {
        FailJobRequest {
            job_key: args.job_key,
            retries: args.retries,
            error_message: args.error_message.to_owned(),
            retry_back_off: args.retry_back_off,
            variables: args.variables.clone(),
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for FailJobArgs {
    type Output = FailJobResponse;

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .fail_job(FailJobRequest::from(&self))
            .await?
            .into_inner())
    }
}
