use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::{Args, Subcommand};
use tonic::transport::Channel;
use zeebe_client::api::gateway_client::GatewayClient;

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
struct PublishMessageArgs {}

pub(crate) async fn handle_publish_command(
    client: &mut GatewayClient<Channel>,
    args: &PublishArgs,
) -> Result<Box<dyn Debug>> {
    todo!()
}
