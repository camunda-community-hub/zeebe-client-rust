use futures::executor::block_on;
use std::time::Duration;
use tonic::transport::Channel;
use zeebe_client::gateway_protocol::gateway_client::GatewayClient;
use zeebe_client::gateway_protocol::TopologyRequest;

use zeebe_client::ZeebeClient;

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

    let grpc_client = GatewayClient::new(channel);



    let zeebe_client = ZeebeClient::default_client();

    // topology
    get_topology_pure_grpc(grpc_client);    
    get_topology_api(zeebe_client);

    

    Ok(())
}


fn get_topology_pure_grpc(mut client : GatewayClient<Channel>) {
    println!("Topology request - pure GRPC");

    let mut request = tonic::Request::new(TopologyRequest {});
    request.set_timeout(Duration::from_secs(1));

    let response = block_on(client.topology(request));

    match response {
        Ok(response) => {
            let topology = response.into_inner();
            println!("SUMMARY: {:?}", topology);
        }
        Err(status) =>  {
            println!("something went wrong: {:?}", status)
        }
    }
}

fn get_topology_api(mut zeebe_client: ZeebeClient) {
    println!("Topology request - API");

    let topology = block_on(zeebe_client.topology());

    match topology {
        Ok(topology) => println!("SUMMARY: {:?}", topology),
        Err(status) => println!("something went wrong: {:?}", status)
    }
}



