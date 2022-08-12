use async_trait::async_trait;
use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::Args;

use zeebe_client::{
    api::{PublishMessageRequest, PublishMessageResponse},
    ZeebeClient,
};

use crate::ExecuteZeebeCommand;

#[derive(Args, Clone, Debug)]
pub(crate) struct PublishMessageArgs {
    name: String,
    #[clap(long)]
    correlation_key: String,
    #[clap(long)]
    message_id: String,
    #[clap(long, required = false, default_value = "")]
    variables: String,
    #[clap(long, required = false, default_value_t = -1)]
    ttl: i64, // todo: should be duration
}

impl From<&PublishMessageArgs> for PublishMessageRequest {
    fn from(args: &PublishMessageArgs) -> Self {
        PublishMessageRequest {
            name: args.name.to_owned(),
            correlation_key: args.correlation_key.to_owned(),
            time_to_live: args.ttl,
            message_id: args.message_id.to_owned(),
            variables: args.variables.to_owned(),
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for PublishMessageArgs {
    type Output = PublishMessageResponse;

    #[tracing::instrument(skip(client))]
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        let args = &self;
        let request: PublishMessageRequest = args.into();
        Ok(client.publish_message(request).await?.into_inner())
    }
}
