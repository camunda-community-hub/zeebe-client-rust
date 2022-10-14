use std::path::PathBuf;

use async_trait::async_trait;
use clap::Args;

use zeebe_client::{
    api::{DeployResourceRequest, DeployResourceResponse, Resource},
    ZeebeClient,
};

use crate::ExecuteZeebeCommand;
use color_eyre::Result;

#[derive(Args)]
pub(crate) struct DeployResourceArgs {
    #[arg(required = true, value_name = "FILE")]
    resources: Vec<PathBuf>,
}
#[async_trait]
impl ExecuteZeebeCommand for DeployResourceArgs {
    type Output = DeployResourceResponse;

    async fn execute(self, client: &mut ZeebeClient) -> Result<Self::Output> {
        Ok(client
            .deploy_resource(DeployResourceRequest::try_from(&self)?)
            .await?
            .into_inner())
    }
}

impl TryFrom<&DeployResourceArgs> for DeployResourceRequest {
    type Error = color_eyre::Report;

    fn try_from(args: &DeployResourceArgs) -> Result<DeployResourceRequest, Self::Error> {
        let mut resources = Vec::with_capacity(args.resources.len());
        for path in &args.resources {
            let resource = Resource {
                name: path
                    .file_name()
                    .expect("resource path should point to a file")
                    .to_str()
                    .expect("file name should be UTF-8")
                    .to_string(),
                content: std::fs::read(path)?,
            };
            resources.push(resource);
        }
        Ok(Self { resources })
    }
}
