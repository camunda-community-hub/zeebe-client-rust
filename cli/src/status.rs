use crate::ExecuteZeebeCommand;
use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{gateway_client::GatewayClient, TopologyRequest, TopologyResponse};

#[derive(Args)]
pub struct StatusArgs {}

#[async_trait]
impl ExecuteZeebeCommand for StatusArgs {
    type Output = TopologyResponse;

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
        Ok(client.topology(TopologyRequest {}).await?.into_inner())
    }
}
