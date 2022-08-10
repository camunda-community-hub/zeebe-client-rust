use std::path::PathBuf;

use async_trait::async_trait;
use clap::Args;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{
    gateway_client::GatewayClient, DeployResourceRequest, DeployResourceResponse, Resource,
};

use crate::ExecuteZeebeCommand;
use color_eyre::Result;

#[derive(Args)]
pub(crate) struct DeployArgs {
    #[clap(required = true, value_parser, value_name = "FILE")]
    resources: Vec<PathBuf>,
}
#[async_trait]
impl ExecuteZeebeCommand for DeployArgs {
    type Output = DeployResourceResponse;

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
            .deploy_resource(DeployResourceRequest::try_from(&self)?)
            .await?
            .into_inner())
    }
}

impl TryFrom<&DeployArgs> for DeployResourceRequest {
    type Error = color_eyre::Report;

    fn try_from(args: &DeployArgs) -> Result<DeployResourceRequest, Self::Error> {
        let mut resources = Vec::with_capacity(args.resources.len());
        for path in &args.resources {
            let resource = Resource {
                name: path
                    .file_name()
                    .expect("resource path should point to a file")
                    .to_str()
                    .expect("file name should be UTF-8")
                    .to_string(),
                content: std::fs::read(path)?,
            };
            resources.push(resource);
        }
        Ok(Self { resources })
    }
}
