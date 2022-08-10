use crate::Debug;
use clap::{Args, Subcommand};
use color_eyre::Result;
use tonic::transport::Channel;
use zeebe_client::api::{gateway_client::GatewayClient, ActivateJobsRequest};

#[derive(Args)]
pub(crate) struct ActivateArgs {
    #[clap(subcommand)]
    resource_type: ActivateResourceType,
}

#[derive(Subcommand)]
enum ActivateResourceType {
    Jobs(ActivateJobsArgs),
}

#[derive(Args)]
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

pub(crate) async fn handle_activate_command(
    client: &mut GatewayClient<Channel>,
    args: &ActivateArgs,
) -> Result<Box<dyn Debug>> {
    match &args.resource_type {
        ActivateResourceType::Jobs(args) => handle_activate_jobs_command(client, args).await,
    }
}

async fn handle_activate_jobs_command(
    client: &mut GatewayClient<Channel>,
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
