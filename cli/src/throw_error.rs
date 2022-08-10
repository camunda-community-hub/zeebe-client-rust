use async_trait::async_trait;
use color_eyre::eyre::Result;


use clap::Args;
use tonic::{
    client::GrpcService,
    codegen::{Body, Bytes, StdError},
};
use zeebe_client::api::{
    gateway_client::GatewayClient, ThrowErrorRequest, ThrowErrorResponse,
};

use crate::ExecuteZeebeCommand;

#[derive(Args)]
pub struct ThrowErrorArgs {
    // the unique job identifier, as obtained when activating the job
    #[clap(short, long)]
    job_key: i64,
    // the error code that will be matched with an error catch event
    #[clap(short = 'c', long)]
    error_code: String,
    // an optional error message that provides additional context
    #[clap(short = 'm', long, default_value = "")]
    error_message: String,
}

impl From<&ThrowErrorArgs> for ThrowErrorRequest {
    fn from(args: &ThrowErrorArgs) -> Self {
        ThrowErrorRequest {
            job_key: args.job_key,
            error_code: args.error_code.to_owned(),
            error_message: args.error_message.to_owned(),
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for ThrowErrorArgs {
    type Output = ThrowErrorResponse;

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
            .throw_error(ThrowErrorRequest::from(&self))
            .await?
            .into_inner())
    }
}
