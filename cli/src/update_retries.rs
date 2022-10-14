use crate::{Debug, ExecuteZeebeCommand};

use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;
use zeebe_client::{
    api::{UpdateJobRetriesRequest, UpdateJobRetriesResponse},
    ZeebeClient,
};

#[derive(Debug, Args)]
pub(crate) struct UpdateRetriesArgs {
    #[arg(short, long)]
    job_key: u64,
    #[arg(short, long)]
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
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .update_job_retries(UpdateJobRetriesRequest::try_from(&self)?)
            .await?
            .into_inner())
    }
}
