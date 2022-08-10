use crate::Debug;

use clap::Args;
use color_eyre::Result;
use tonic::transport::Channel;
use zeebe_client::api::{gateway_client::GatewayClient, UpdateJobRetriesRequest};

#[derive(Args)]
pub(crate) struct UpdateRetriesArgs {
    #[clap(long)]
    job_key: u64,
    #[clap(long)]
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

pub(crate) async fn handle_set_retries_command(
    client: &mut GatewayClient<Channel>,
    args: &UpdateRetriesArgs,
) -> Result<Box<dyn Debug>> {
    let request: UpdateJobRetriesRequest = args.try_into()?;
    Ok(Box::new(
        client.update_job_retries(request).await?.into_inner(),
    ))
}
