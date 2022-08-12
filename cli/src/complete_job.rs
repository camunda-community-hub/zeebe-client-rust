use crate::{Debug, ExecuteZeebeCommand};
use async_trait::async_trait;
use clap::Args;
use color_eyre::eyre::Result;
use zeebe_client::{
    api::{CompleteJobRequest, CompleteJobResponse},
    ZeebeClient,
};

#[derive(Args, Clone, Debug)]
pub(crate) struct CompleteJobArgs {
    #[clap(short, long)]
    job_key: i64,

    #[clap(long, required = false, default_value = "")]
    variables: String,
}

impl From<&CompleteJobArgs> for CompleteJobRequest {
    fn from(args: &CompleteJobArgs) -> Self {
        CompleteJobRequest {
            job_key: args.job_key,
            variables: args.variables.clone(),
        }
    }
}

#[async_trait]
impl ExecuteZeebeCommand for CompleteJobArgs {
    type Output = CompleteJobResponse;

    #[tracing::instrument(skip(client))]
    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        let args = &self;
        let request: CompleteJobRequest = args.into();
        Ok(client.complete_job(request).await?.into_inner())
    }
}
