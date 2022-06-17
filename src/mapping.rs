use super::*;

pub fn map_topology_response(response: gateway_protocol::TopologyResponse) -> Topology {
    let brokers = response
        .brokers
        .iter()
        .map(|broker| map_broker_info(broker))
        .collect::<Vec<BrokerInfo>>();

    Topology {
        brokers,
        cluster_size: response.cluster_size,
        partition_count: response.partitions_count,
        replication_factor: response.replication_factor,
        gateway_version: response.gateway_version.clone(),
    }
}

fn map_broker_info(broker_info: &gateway_protocol::BrokerInfo) -> BrokerInfo {
    let partitions = broker_info
        .partitions
        .iter()
        .map(|partition| map_partition(partition))
        .collect::<Vec<PartitionInfo>>();

    BrokerInfo {
        node_id: broker_info.node_id,
        host: broker_info.host.clone(),
        port: broker_info.port,
        version: broker_info.version.clone(),
        partitions,
    }
}

fn map_partition(partition: &gateway_protocol::Partition) -> PartitionInfo {
    PartitionInfo {
        partition_id: partition.partition_id,
        role: match gateway_protocol::partition::PartitionBrokerRole::from_i32(partition.role) {
            Some(gateway_protocol::partition::PartitionBrokerRole::Leader) => {
                PartitionBrokerRole::LEADER
            }
            Some(gateway_protocol::partition::PartitionBrokerRole::Follower) => {
                PartitionBrokerRole::FOLLOWER
            }
            Some(gateway_protocol::partition::PartitionBrokerRole::Inactive) => {
                PartitionBrokerRole::INACTIVE
            }
            None => {
                panic!("Unknown value {:?}", partition.role);
            }
        },
        health: match gateway_protocol::partition::PartitionBrokerHealth::from_i32(partition.health)
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
            None => {
                panic!("Unknown value {:?}", partition.health);
            }
        },
    }
}
