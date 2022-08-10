use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::{Args, Subcommand};
use tonic::transport::Channel;
use zeebe_client::api::{gateway_client::GatewayClient, PublishMessageRequest};

#[derive(Args, Clone, Debug)]
pub(crate) struct PublishArgs {
    #[clap(subcommand)]
    resource_type: PublishResourceType,
}

#[derive(Subcommand, Clone, Debug)]
enum PublishResourceType {
    Message(PublishMessageArgs),
}

#[derive(Args, Clone, Debug)]
struct PublishMessageArgs {
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

pub(crate) async fn handle_publish_command(
    client: &mut GatewayClient<Channel>,
    args: &PublishArgs,
) -> Result<Box<dyn Debug>> {
    match &args.resource_type {
        PublishResourceType::Message(args) => handle_publish_message_command(client, args).await,
    }
}

async fn handle_publish_message_command(
    client: &mut GatewayClient<Channel>,
    args: &PublishMessageArgs,
) -> Result<Box<dyn Debug>> {
    let request: PublishMessageRequest = args.into();
    Ok(Box::new(
        client.publish_message(request).await?.into_inner(),
    ))
}
