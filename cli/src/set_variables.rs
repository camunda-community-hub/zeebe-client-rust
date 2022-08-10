use std::path::PathBuf;

use crate::ExecuteZeebeCommand;
use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{gateway_client::GatewayClient, SetVariablesRequest, SetVariablesResponse};

#[derive(Args)]

pub(crate) struct SetVariablesArgs {
    element_instance_key: i64,
    #[clap(long)]
    local: bool,
    #[clap(long, value_parser, group = "value")]
    path: Option<PathBuf>,
    #[clap(long, group = "value")]
    json: Option<String>,
}

impl TryFrom<SetVariablesArgs> for SetVariablesRequest {
    type Error = color_eyre::Report;

    fn try_from(args: SetVariablesArgs) -> Result<SetVariablesRequest, Self::Error> {
        let variables = if let Some(path) = &args.path {
            std::fs::read_to_string(path)?
        } else if let Some(json) = args.json {
            json
        } else {
            unreachable!()
        };
        Ok(Self {
            element_instance_key: args.element_instance_key,
            variables,
            local: args.local,
        })
    }
}

#[async_trait]
impl ExecuteZeebeCommand for SetVariablesArgs {
    type Output = SetVariablesResponse;

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
            .set_variables(SetVariablesRequest::try_from(self)?)
            .await?
            .into_inner())
    }
}
