use crate::{Debug, ExecuteZeebeCommand};

use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{
    gateway_client::GatewayClient, UpdateJobRetriesRequest, UpdateJobRetriesResponse,
};

#[derive(Debug, Args)]
pub(crate) struct UpdateRetriesArgs {
    #[clap(long)]
    job_key: u64,
    #[clap(long)]
    retries: u32,
}

impl TryFrom<&UpdateRetriesArgs> for UpdateJobRetriesRequest {
    type Error = std::num::TryFromIntError;
    fn try_from(
        args: &UpdateRetriesArgs,
    ) -> Result<UpdateJobRetriesRequest, std::num::TryFromIntError> {
        Ok(UpdateJobRetriesRequest {
            job_key: args.job_key.try_into()?,
            retries: args.retries.try_into()?,
        })
    }
}

#[async_trait]
impl ExecuteZeebeCommand for UpdateRetriesArgs {
    type Output = UpdateJobRetriesResponse;

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
        Ok(client
            .update_job_retries(UpdateJobRetriesRequest::try_from(&self)?)
            .await?
            .into_inner())
    }
}
