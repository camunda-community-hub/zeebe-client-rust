use futures::executor::block_on;
use std::path::Path;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use tonic::transport::Channel;
use zeebe_client::gateway_protocol::gateway_client::GatewayClient;
use zeebe_client::gateway_protocol::TopologyRequest;
use zeebe_client::gateway_protocol::DeployProcessRequest;
use zeebe_client::gateway_protocol::ProcessRequestObject;

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

    let mut grpc_client = GatewayClient::new(channel);



    let mut zeebe_client = ZeebeClient::default_client();

    // topology
    get_topology_pure_grpc(&mut grpc_client);    
    get_topology_api(&mut zeebe_client);

    // deploy process
    deploy_process_pure_grpc(&mut grpc_client);
    

    Ok(())
}


fn get_topology_pure_grpc(client : &mut GatewayClient<Channel>) {
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

fn get_topology_api(zeebe_client: &mut ZeebeClient) {
    println!("Topology request - API");

    let topology = block_on(zeebe_client.topology());

    match topology {
        Ok(topology) => println!("SUMMARY: {:?}", topology),
        Err(status) => println!("something went wrong: {:?}", status)
    }
}

fn deploy_process_pure_grpc(client : &mut GatewayClient<Channel>) {
    println!("Deploy process request - pure GRPC");

    let process_definition = NamedResource::from_file( Path::new("resources/start-end.bpmn")).to_process_request_object();

    let processes = vec![process_definition];

    let deploy_process_message = DeployProcessRequest {
        processes,
    };

    let response = block_on(client.deploy_process(deploy_process_message));

    match response {
        Ok(response) => {
            let response = response.into_inner();
            println!("SUMMARY: {:?}", response);
        }
        Err(status) =>  {
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

struct NamedResource {
    name: String,
    content : Vec<u8>,
}

impl NamedResource {

    fn from_file(filename: &Path) -> NamedResource {

        let name = filename.file_name().unwrap().to_str().unwrap().to_string();
        let content = get_file_as_byte_vec(filename);

        NamedResource {
            name,
            content
        }
    }

    fn to_process_request_object(&self) -> ProcessRequestObject {
        ProcessRequestObject { 
            name: self.name.clone(),
            definition: self.content.clone(),
        }
    }
}