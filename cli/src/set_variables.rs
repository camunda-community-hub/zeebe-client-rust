use std::path::PathBuf;

use crate::ExecuteZeebeCommand;
use async_trait::async_trait;
use clap::Args;
use color_eyre::Result;
use zeebe_client::{
    api::{SetVariablesRequest, SetVariablesResponse},
    ZeebeClient,
};

#[derive(Args)]

pub(crate) struct SetVariablesArgs {
    element_instance_key: i64,
    #[clap(long)]
    local: bool,
    #[clap(long, value_parser, group = "value")]
    path: Option<PathBuf>,
    #[clap(long, group = "value")]
    json: Option<String>,
}

impl TryFrom<SetVariablesArgs> for SetVariablesRequest {
    type Error = color_eyre::Report;

    fn try_from(args: SetVariablesArgs) -> Result<SetVariablesRequest, Self::Error> {
        let variables = if let Some(path) = &args.path {
            std::fs::read_to_string(path)?
        } else if let Some(json) = args.json {
            json
        } else {
            unreachable!()
        };
        Ok(Self {
            element_instance_key: args.element_instance_key,
            variables,
            local: args.local,
        })
    }
}

#[async_trait]
impl ExecuteZeebeCommand for SetVariablesArgs {
    type Output = SetVariablesResponse;

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .set_variables(SetVariablesRequest::try_from(self)?)
            .await?
            .into_inner())
    }
}
