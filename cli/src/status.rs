use crate::ExecuteZeebeCommand;
use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;

use zeebe_client::{
    api::{TopologyRequest, TopologyResponse},
    ZeebeClient,
};

#[derive(Args)]
pub struct StatusArgs {}

#[async_trait]
impl ExecuteZeebeCommand for StatusArgs {
    type Output = TopologyResponse;

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client.topology(TopologyRequest {}).await?.into_inner())
    }
}
