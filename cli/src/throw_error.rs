use async_trait::async_trait;
use color_eyre::eyre::Result;

use clap::Args;
use zeebe_client::{
    api::{ThrowErrorRequest, ThrowErrorResponse},
    ZeebeClient,
};

use crate::ExecuteZeebeCommand;

#[derive(Args)]
pub(crate) struct ThrowErrorArgs {
    // the unique job identifier, as obtained when activating the job
    #[clap(short, long)]
    job_key: i64,
    // the error code that will be matched with an error catch event
    #[clap(short = 'c', long)]
    error_code: String,
    // an optional error message that provides additional context
    #[clap(long, default_value = "")]
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

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .throw_error(ThrowErrorRequest::from(&self))
            .await?
            .into_inner())
    }
}
