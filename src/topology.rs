#[derive(Eq, PartialEq, Debug)]
pub enum PartitionBrokerHealth {
    HEALTHY,
    UNHEALTHY,
    DEAD,
}

#[derive(Eq, PartialEq, Debug)]
pub enum PartitionBrokerRole {
    LEADER,
    FOLLOWER,
    INACTIVE,
}

#[derive(Eq, PartialEq, Debug)]
pub struct PartitionInfo {
    pub partition_id: i32,
    pub role: PartitionBrokerRole,
    pub health: PartitionBrokerHealth,
}

impl PartitionInfo {
    pub fn is_leader(&self) -> bool {
        return self.role == PartitionBrokerRole::LEADER;
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct BrokerInfo {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
    pub version: String,
    pub partitions: Vec<PartitionInfo>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Topology {
    pub brokers: Vec<BrokerInfo>,
    pub cluster_size: i32,
    pub partition_count: i32,
    pub replication_factor: i32,
    pub gateway_version: String,
}
