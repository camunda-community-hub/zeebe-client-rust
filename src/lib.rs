extern crate core;

pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}

pub mod deploy;
mod mapping;
pub mod topology;

use deploy::DeployResourceResponse;
use futures::executor::block_on;
use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::DeployResourceRequest;
use gateway_protocol::TopologyRequest;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tonic::transport::Channel;
use topology::{BrokerInfo, PartitionBrokerHealth, PartitionBrokerRole, PartitionInfo, Topology};

pub struct ZeebeClient {
    client: GatewayClient<Channel>,
}

impl ZeebeClient {
    pub fn default_client() -> ZeebeClient {
        let channel = block_on(
            Channel::from_static("http://[::1]:26500")
                //.tls_config(tls)?
                .connect(),
        );

        let channel = channel.unwrap();

        let client = GatewayClient::new(channel);

        ZeebeClient { client }
    }

    pub async fn topology(&mut self) -> Result<Topology, tonic::Status> {
        let response = self.client.topology(TopologyRequest {}).await;

        return response.map(|response| mapping::map_topology_response(response.into_inner()));
    }

    pub async fn deploy_resources(&mut self,
        resources: Vec<&Path>
    ) -> Result<DeployResourceResponse, tonic::Status> {
        let resources = resources.iter().map(|file| new_resource(file)).collect();

        let request = DeployResourceRequest {
            resources
        };

        let response= self.client.deploy_resource(request).await;

        return response.map(|response| mapping::map_deploy_resource_response(response.into_inner()));
    }
}

fn new_resource(filename: &Path) -> gateway_protocol::Resource {
    let name = filename.file_name().unwrap().to_str().unwrap().to_string();

    let mut f = File::open(filename).expect("File not found");
    let mut content = Vec::new();

    // read the whole file
    f.read_to_end(&mut content).expect("Error on read");

    gateway_protocol::Resource { name, content }
}
