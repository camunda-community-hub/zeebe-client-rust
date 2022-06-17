pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}
use futures::executor::block_on;
use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::TopologyRequest;
use std::time::Duration;
use tonic::transport::{Channel, Error};

use zeebe_client::topology::{PartitionBrokerHealth, PartitionBrokerRole, PartitionInfo};
use zeebe_client::ZeebeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zeebe_client = ZeebeClient::default_client();

    println!("Topology request - pure GRPC");
    get_topology_pure_grpc().await?;

    println!("Topology request - API");
    get_topology_api(zeebe_client);

    let part = PartitionInfo {
        partition_id: 0,
        role: PartitionBrokerRole::LEADER,
        health: PartitionBrokerHealth::HEALTHY,
    };

    println!("{:?}", part);

    Ok(())
}

async fn get_topology_pure_grpc() -> Result<bool, Error> {
    //let server_root_ca_cert = tokio::fs::read("examples/data/tls/ca.pem").await?;
    //let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);
    //let client_cert = tokio::fs::read("examples/data/tls/client1.pem").await?;
    //let client_key = tokio::fs::read("examples/data/tls/client1.key").await?;
    //let client_identity = Identity::from_pem(client_cert, client_key);

    //let tls = ClientTlsConfig::new()
    //    .domain_name("localhost")
    //    .ca_certificate(server_root_ca_cert)
    //    .identity(client_identity);

    let channel = Channel::from_static("http://[::1]:26500")
        //.tls_config(tls)?
        .timeout(Duration::from_secs(5))
        .connect()
        .await?;

    let mut client = GatewayClient::new(channel);

    let mut request = tonic::Request::new(TopologyRequest {});
    request.set_timeout(Duration::from_secs(1));

    let response = client.topology(request).await;

    match response {
        Ok(response) => {
            let topology = response.into_inner();
            println!("SUMMARY: {:?}", topology);
        }
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(true)
}

fn get_topology_api(mut zeebe_client: ZeebeClient) {
    let topology = block_on(zeebe_client.topology());

    println!("SUMMARY: {:?}", topology);
}
