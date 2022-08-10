use async_trait::async_trait;
use color_eyre::eyre::Result;


use crate::ExecuteZeebeCommand;
use clap::Args;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{
    gateway_client::GatewayClient, FailJobRequest, FailJobResponse,
};
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

#[async_trait]
impl ExecuteZeebeCommand for FailJobArgs {
    type Output = FailJobResponse;

    async fn execute<Service: Send>(
        self,
        client: &mut GatewayClient<Service>,
    ) -> Result<Self::Output>
    where
        Service: tonic::client::GrpcService<tonic::body::BoxBody>,
        Service::Error: Into<StdError>,
        Service::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <Service::ResponseBody as Body>::Error: Into<StdError> + Send,
        <Service as GrpcService<tonic::body::BoxBody>>::Future: Send,
    {
        Ok(client
            .fail_job(FailJobRequest::from(&self))
            .await?
            .into_inner())
    }
}
