extern crate core;

pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}

mod mapping;
pub mod topology;

use futures::executor::block_on;
use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::TopologyRequest;
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

        match response {
            Ok(response) => Ok(mapping::map_topology_response(response.into_inner())),
            Err(e) => Err(e),
        }
    }
}
