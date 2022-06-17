use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::TopologyRequest;

pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GatewayClient::connect("http://[::1]:26500").await?;

    let response = client.topology(TopologyRequest {}).await;

    match response {
        Ok(response) => {
            let topology = response.into_inner();
            println!("SUMMARY: {:?}", topology);
            println!("ClusterSize: {:?}", topology.cluster_size)
        }
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}
