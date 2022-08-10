use crate::{Debug, ExecuteZeebeCommand};
use async_trait::async_trait;
use clap::{Args, Subcommand};
use color_eyre::eyre::Result;
use tonic::{codegen::{StdError, Body, Bytes}, client::GrpcService};
use zeebe_client::api::{CreateProcessInstanceRequest, gateway_client::GatewayClient, CreateProcessInstanceWithResultRequest};

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

#[async_trait]
impl ExecuteZeebeCommand for CreateArgs {
    type Output = Box<dyn Debug>;

    #[tracing::instrument(skip(client))]
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
        match &self.resource_type {
            CreateResourceType::Instance(args) => {
                handle_create_instance_command(client, args).await
            }
        }
    }
}

async fn handle_create_instance_command<Service: Send>(
    client: &mut GatewayClient<Service>,
    args: &CreateInstanceArgs,
) -> Result<Box<dyn Debug>>
where
    Service: tonic::client::GrpcService<tonic::body::BoxBody>,
    Service::Error: Into<StdError>,
    Service::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <Service::ResponseBody as Body>::Error: Into<StdError> + Send,
    <Service as GrpcService<tonic::body::BoxBody>>::Future: Send,
{
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