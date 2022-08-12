use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;

use zeebe_client::{
    api::{ResolveIncidentRequest, ResolveIncidentResponse},
    ZeebeClient,
};

use crate::ExecuteZeebeCommand;

#[derive(Args)]
pub struct ResolveIncidentArgs {
    incident_key: i64,
}

#[async_trait]
impl ExecuteZeebeCommand for ResolveIncidentArgs {
    type Output = ResolveIncidentResponse;

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .resolve_incident(ResolveIncidentRequest {
                incident_key: self.incident_key,
            })
            .await?
            .into_inner())
    }
}
