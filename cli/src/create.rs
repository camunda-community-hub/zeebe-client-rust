use crate::Debug;
use clap::{Args, Subcommand};
use color_eyre::eyre::Result;

use zeebe_client::{
    api::{CreateProcessInstanceRequest, CreateProcessInstanceWithResultRequest},
    ZeebeClient,
};

#[derive(Args, Clone, Debug)]
pub(crate) struct CreateArgs {
    #[clap(subcommand)]
    resource_type: CreateResourceType,
}

#[derive(Subcommand, Clone, Debug)]
enum CreateResourceType {
    Instance(CreateInstanceArgs),
}

#[derive(Args, Clone, Debug)]
struct CreateInstanceArgs {
    process: i64,

    #[clap(long, required = false)]
    with_results: bool,
    #[clap(long, required = false, default_value = "")]
    variables: String,
    #[clap(long, required = false, default_value_t = -1)]
    version: i32,
}

pub(crate) async fn handle_create_command(
    client: &mut ZeebeClient,
    args: &CreateArgs,
) -> Result<Box<dyn Debug>> {
    match &args.resource_type {
        CreateResourceType::Instance(args) => handle_create_instance_command(client, args).await,
    }
}

impl From<&CreateInstanceArgs> for CreateProcessInstanceRequest {
    fn from(args: &CreateInstanceArgs) -> Self {
        CreateProcessInstanceRequest {
            process_definition_key: args.process,
            bpmn_process_id: String::new(),
            version: args.version,
            variables: args.variables.clone(),
            start_instructions: vec![],
        }
    }
}

async fn handle_create_instance_command(
    client: &mut ZeebeClient,
    args: &CreateInstanceArgs,
) -> Result<Box<dyn Debug>> {
    let request: CreateProcessInstanceRequest = args.into();
    match args.with_results {
        true => Ok(Box::new(
            client
                .create_process_instance_with_result(CreateProcessInstanceWithResultRequest {
                    request: Some(request),
                    ..Default::default()
                })
                .await?
                .into_inner(),
        )),
        false => Ok(Box::new(
            client.create_process_instance(request).await?.into_inner(),
        )),
    }
}
