use crate::{Debug, ExecuteZeebeCommand};
use async_trait::async_trait;
use clap::Args;
use color_eyre::eyre::Result;
use zeebe_client::{
    api::{CreateProcessInstanceRequest, CreateProcessInstanceWithResultRequest},
    ZeebeClient,
};

#[derive(Args, Clone, Debug)]
pub(crate) struct CreateProcessInstanceArgs {
    process_instance_key: i64,

    #[clap(long, required = false)]
    with_results: bool,
    #[clap(long, required = false, default_value = "")]
    variables: String,
    #[clap(long, required = false, default_value_t = -1)]
    version: i32,
}

impl From<&CreateProcessInstanceArgs> for CreateProcessInstanceRequest {
    fn from(args: &CreateProcessInstanceArgs) -> Self {
        CreateProcessInstanceRequest {
            process_definition_key: args.process_instance_key,
            bpmn_process_id: String::new(),
            version: args.version,
            variables: args.variables.clone(),
            start_instructions: vec![],
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for CreateProcessInstanceArgs {
    type Output = Box<dyn Debug>;

    #[tracing::instrument(skip(client))]
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        handle_create_instance_command(client, &self).await
    }
}

async fn handle_create_instance_command(
    client: &mut ZeebeClient,
    args: &CreateProcessInstanceArgs,
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
