use color_eyre::eyre::Result;
use std::fmt::Debug;

use clap::Args;

use zeebe_client::{api::ThrowErrorRequest, ZeebeClient};

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

pub async fn handle_command(
    client: &mut ZeebeClient,
    args: &ThrowErrorArgs,
) -> Result<Box<dyn Debug>> {
    let request: ThrowErrorRequest = args.into();
    Ok(Box::new(client.throw_error(request).await?.into_inner()))
}
