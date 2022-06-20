use futures::executor::block_on;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Duration;
use tonic::transport::Channel;
use zeebe_client::gateway_protocol::gateway_client::GatewayClient;
use zeebe_client::gateway_protocol::DeployResourceRequest;
use zeebe_client::gateway_protocol::Resource;
use zeebe_client::gateway_protocol::TopologyRequest;

use zeebe_client::ZeebeClient;

const START_END_PROCESS_FILE: &'static str = "resources/start-end.bpmn";

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

    let mut grpc_client = GatewayClient::new(channel);

    let mut zeebe_client = ZeebeClient::default_client();

    // topology
    get_topology_pure_grpc(&mut grpc_client);
    get_topology_api(&mut zeebe_client);

    // deploy process
    deploy_resource_pure_grpc(&mut grpc_client);
    deploy_resource_api(&mut zeebe_client);

    Ok(())
}

fn get_topology_pure_grpc(client: &mut GatewayClient<Channel>) {
    println!("Topology request - pure GRPC");

    let mut request = tonic::Request::new(TopologyRequest {});
    request.set_timeout(Duration::from_secs(1));

    let response = block_on(client.topology(request));

    match response {
        Ok(response) => {
            let topology = response.into_inner();
            println!("SUMMARY: {:?}", topology);
        }
        Err(status) => {
            println!("something went wrong: {:?}", status)
        }
    }
}

fn get_topology_api(zeebe_client: &mut ZeebeClient) {
    println!("Topology request - API");

    let topology = block_on(zeebe_client.topology());

    match topology {
        Ok(topology) => println!("SUMMARY: {:?}", topology),
        Err(status) => println!("something went wrong: {:?}", status),
    }
}

fn deploy_resource_pure_grpc(client: &mut GatewayClient<Channel>) {
    println!("Deploy resource request - pure GRPC");

    let resource_definition = new_resource(Path::new(START_END_PROCESS_FILE));

    let resources = vec![resource_definition];

    let request = DeployResourceRequest { resources };

    let response = block_on(client.deploy_resource(request));

    match response {
        Ok(response) => {
            let response = response.into_inner();
            println!("SUMMARY: {:?}", response);
        }
        Err(status) => {
            println!("something went wrong: {:?}", status)
        }
    }
}

fn get_file_as_byte_vec(filename: &Path) -> Vec<u8> {
    let mut f = File::open(filename).expect("File not found");
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).expect("Error on read");

    buffer
}

fn new_resource(filename: &Path) -> Resource {
    let name = filename.file_name().unwrap().to_str().unwrap().to_string();
    let content = get_file_as_byte_vec(filename);

    Resource { name, content }
}

fn deploy_resource_api(zeebe_client: &mut ZeebeClient) {
    println!("Deploy resource request - API");

    let response = block_on(zeebe_client.deploy_resources(vec![Path::new(START_END_PROCESS_FILE)]));

    match response {
        Ok(topology) => println!("SUMMARY: {:?}", topology),
        Err(status) => println!("something went wrong: {:?}", status),
    }
}
