use std::path::PathBuf;

use clap::Args;
use zeebe_client::api::SetVariablesRequest;

#[derive(Args)]

pub struct SetVariablesArgs {
    element_instance_key: i64,
    #[clap(long)]
    local: bool,
    #[clap(long, value_parser, group = "value")]
    path: Option<PathBuf>,
    #[clap(long, group = "value")]
    json: Option<String>,
}

impl TryInto<SetVariablesRequest> for SetVariablesArgs {
    type Error = color_eyre::Report;

    fn try_into(self) -> Result<SetVariablesRequest, Self::Error> {
        let variables = if let Some(path) = self.path {
            std::fs::read_to_string(path)?
        } else if let Some(json) = self.json {
            json
        } else {
            unreachable!()
        };
        Ok(SetVariablesRequest {
            element_instance_key: self.element_instance_key,
            variables,
            local: self.local,
        })
    }
}
