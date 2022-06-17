extern crate core;

pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}

pub mod topology;

use crate::topology::{
    BrokerInfo, PartitionBrokerHealth, PartitionBrokerRole, PartitionInfo, Topology,
};
use futures::executor::block_on;
use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::TopologyRequest;
use tonic::transport::Channel;

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
            Ok(response) => Ok(response.into_inner().to_external_representation()),
            Err(e) => Err(e),
        }
    }
}

impl gateway_protocol::TopologyResponse {
    fn to_external_representation(&self) -> Topology {
        let brokers = self
            .brokers
            .iter()
            .map(|broker| broker.to_external_representation())
            .collect::<Vec<BrokerInfo>>();

        Topology {
            brokers,
            cluster_size: self.cluster_size,
            partition_count: self.partitions_count,
            replication_factor: self.replication_factor,
            gateway_version: self.gateway_version.clone(),
        }
    }
}

impl gateway_protocol::BrokerInfo {
    fn to_external_representation(&self) -> BrokerInfo {
        let partitions = self
            .partitions
            .iter()
            .map(|partition| partition.to_external_representation())
            .collect::<Vec<PartitionInfo>>();

        BrokerInfo {
            node_id: self.node_id,
            host: self.host.clone(),
            port: self.port,
            version: self.version.clone(),
            partitions,
        }
    }
}

impl gateway_protocol::Partition {
    fn to_external_representation(&self) -> PartitionInfo {
        PartitionInfo {
            partition_id: self.partition_id,
            role: match gateway_protocol::partition::PartitionBrokerRole::from_i32(self.role) {
                Some(gateway_protocol::partition::PartitionBrokerRole::Leader) => {
                    PartitionBrokerRole::LEADER
                }
                Some(gateway_protocol::partition::PartitionBrokerRole::Follower) => {
                    PartitionBrokerRole::FOLLOWER
                }
                Some(gateway_protocol::partition::PartitionBrokerRole::Inactive) => {
                    PartitionBrokerRole::INACTIVE
                }
                other => {
                    panic!("Unknown value {:?}", other);
                }
            },
            health: match gateway_protocol::partition::PartitionBrokerHealth::from_i32(self.health)
            {
                Some(gateway_protocol::partition::PartitionBrokerHealth::Healthy) => {
                    PartitionBrokerHealth::HEALTHY
                }
                Some(gateway_protocol::partition::PartitionBrokerHealth::Unhealthy) => {
                    PartitionBrokerHealth::UNHEALTHY
                }
                Some(gateway_protocol::partition::PartitionBrokerHealth::Dead) => {
                    PartitionBrokerHealth::DEAD
                }
                other => {
                    panic!("Unknown value {:?}", other);
                }
            },
        }
    }
}
