use crate::deploy::Metadata;

use super::*;

pub fn map_topology_response(response: gateway_protocol::TopologyResponse) -> Topology {
    let brokers = response
        .brokers
        .iter()
        .map(|broker| map_broker_info(broker))
        .collect();

    Topology {
        brokers,
        cluster_size: response.cluster_size,
        partition_count: response.partitions_count,
        replication_factor: response.replication_factor,
        gateway_version: response.gateway_version.clone(),
    }
}

pub fn map_deploy_resource_response(
    response: gateway_protocol::DeployResourceResponse,
) -> DeployResourceResponse {
    let deployments = response
        .deployments
        .iter()
        .map(|deployment| deployment.metadata.to_owned())
        .map(|opt_metadata| opt_metadata.map(|metadata| map_metadata(metadata)))
        .collect();

    DeployResourceResponse {
        key: response.key,
        deployments,
    }
}

fn map_broker_info(broker_info: &gateway_protocol::BrokerInfo) -> BrokerInfo {
    let partitions = broker_info
        .partitions
        .iter()
        .map(|partition| map_partition(partition))
        .collect();

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

fn map_metadata(metadata: gateway_protocol::deployment::Metadata) -> Metadata {
    match metadata {
        gateway_protocol::deployment::Metadata::Process(process_metadata) => {
            Metadata::Process(deploy::ProcessMetadata {
                bpmn_process_id: process_metadata.bpmn_process_id,
                version: process_metadata.version,
                process_definition_key: process_metadata.process_definition_key,
                resource_name: process_metadata.resource_name,
            })
        }
        gateway_protocol::deployment::Metadata::Decision(decision_metadata) => {
            Metadata::Decision(deploy::DecisionMetadata {
                dmn_decision_id: decision_metadata.dmn_decision_id,
                dmn_decision_name: decision_metadata.dmn_decision_name,
                version: decision_metadata.version,
                decision_key: decision_metadata.decision_key,
                dmn_decision_requirements_id: decision_metadata.dmn_decision_requirements_id,
                decision_requirements_key: decision_metadata.decision_requirements_key,
            })
        }
        gateway_protocol::deployment::Metadata::DecisionRequirements(
            decision_requirements_metadata,
        ) => Metadata::DecisionRequirements(deploy::DecisionRequirementsMetadata {
            dmn_decision_requirements_id: decision_requirements_metadata
                .dmn_decision_requirements_id,
            dmn_decision_requirements_name: decision_requirements_metadata
                .dmn_decision_requirements_name,
            version: decision_requirements_metadata.version,
            decision_requirements_key: decision_requirements_metadata.decision_requirements_key,
            resource_name: decision_requirements_metadata.resource_name,
        }),
    }
}
