use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::Args;
use tonic::transport::Channel;
use zeebe_client::api::{gateway_client::GatewayClient, FailJobRequest};

#[derive(Args)]
pub struct FailJobArgs {
    // the unique job identifier, as obtained when activating the job
    #[clap(required = true, short, long)]
    job_key: i64,
    // the amount of retries the job should have left
    #[clap(required = true, short, long)]
    retries: i32,
    // an optional message describing why the job failed
    // this is particularly useful if a job runs out of retries and an incident is raised,
    // as it this message can help explain why an incident was raised
    #[clap(required = false, short, long, default_value = "")]
    error_message: String,
    // the back off timeout for the next retry
    #[clap(required = false, short = 'b', long, default_value = "0")]
    retry_back_off: i64,
}

impl From<&FailJobArgs> for FailJobRequest {
    fn from(args: &FailJobArgs) -> Self {
        FailJobRequest {
            job_key: args.job_key,
            retries: args.retries,
            error_message: args.error_message.to_owned(),
            retry_back_off: args.retry_back_off,
        }
    }
}

pub async fn handle_command(
    client: &mut GatewayClient<Channel>,
    args: &FailJobArgs,
) -> Result<Box<dyn Debug>> {
    let request: FailJobRequest = args.into();
    Ok(Box::new(client.fail_job(request).await?.into_inner()))
}
