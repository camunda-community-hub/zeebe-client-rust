use crate::{Debug, ExecuteZeebeCommand};
use async_trait::async_trait;
use clap::{Args, Subcommand};
use color_eyre::Result;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::{api::{gateway_client::GatewayClient, ActivateJobsRequest}, ZeebeClient};

#[derive(Debug, Args)]
pub(crate) struct ActivateArgs {
    #[clap(subcommand)]
    resource_type: ActivateResourceType,
}

#[derive(Debug, Subcommand)]
enum ActivateResourceType {
    Jobs(ActivateJobsArgs),
}

#[derive(Args, Debug)]
struct ActivateJobsArgs {
    job_type: String,
    #[clap(long, default_value_t = 1)]
    max_jobs_to_activate: u32,
    #[clap(long, default_value_t = 5 * 60 * 1000)]
    job_timeout: u64, // todo: should be duration
    worker: String,
    #[clap(long, required = false)]
    variables: Vec<String>,
}

impl From<&ActivateJobsArgs> for ActivateJobsRequest {
    fn from(args: &ActivateJobsArgs) -> Self {
        ActivateJobsRequest {
            r#type: args.job_type.to_owned(),
            worker: args.worker.to_owned(),
            timeout: args.job_timeout as i64,
            max_jobs_to_activate: args.max_jobs_to_activate as i32,
            fetch_variable: args.variables.to_owned(),
            request_timeout: Default::default(),
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for ActivateArgs {
    type Output = Box<dyn Debug>;

    #[tracing::instrument(skip(client))]
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        match &self.resource_type {
            ActivateResourceType::Jobs(args) => handle_activate_jobs_command(client, args).await,
        }
    }
}

async fn handle_activate_jobs_command(
    client: &mut ZeebeClient,
    args: &ActivateJobsArgs,
) -> Result<Box<dyn Debug>> {
    let request: ActivateJobsRequest = args.into();
    let mut stream = client.activate_jobs(request).await?.into_inner();
    let mut result = Vec::with_capacity(args.max_jobs_to_activate.try_into().unwrap());
    while let Some(response) = stream.message().await? {
        result.push(response);
    }
    Ok(Box::new(result))
}
