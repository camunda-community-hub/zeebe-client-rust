use async_trait::async_trait;
use color_eyre::eyre::Result;

use clap::Args;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{
    gateway_client::GatewayClient, CancelProcessInstanceRequest, CancelProcessInstanceResponse,
};

use crate::ExecuteZeebeCommand;

#[derive(Args)]
pub struct CancelProcessInstanceArgs {
    process_instance_key: i64,
}

impl From<&CancelProcessInstanceArgs> for CancelProcessInstanceRequest {
    fn from(args: &CancelProcessInstanceArgs) -> Self {
        CancelProcessInstanceRequest {
            process_instance_key: args.process_instance_key,
        }
    }
}


#[async_trait]
impl ExecuteZeebeCommand for CancelProcessInstanceArgs {
    type Output = CancelProcessInstanceResponse;
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
            .cancel_process_instance(CancelProcessInstanceRequest::from(&self))
            .await?
            .into_inner())
    }
}
