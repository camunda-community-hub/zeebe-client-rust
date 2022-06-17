pub mod gateway_protocol {
    tonic::include_proto!("gateway_protocol");
}

use gateway_protocol::gateway_client::GatewayClient;
use gateway_protocol::TopologyRequest;
use std::time::Duration;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

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

    let mut request = tonic::Request::new(TopologyRequest {
    });
    request.set_timeout(Duration::from_secs(1));

    let response = client.topology(request).await;

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
