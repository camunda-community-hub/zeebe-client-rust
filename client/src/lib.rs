use std::str::FromStr;

use api::gateway_client::GatewayClient;
use thiserror::Error;
use tonic::{
    codegen::http::uri::InvalidUri,
    transport::{self, Channel, Uri},
};

mod generated_api {
    tonic::include_proto!("gateway_protocol");
}

pub mod api {
    pub use super::generated_api::*;
}

pub enum Connection {
    Address(String),
    HostPort(String, u16),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    Transport(#[from] transport::Error),
    #[error(transparent)]
    Uri(#[from] InvalidUri),
}

pub async fn connect(conn: Connection) -> Result<GatewayClient<Channel>, ConnectionError> {
    let uri = match conn {
        Connection::Address(addr) => Uri::from_str(&addr),
        Connection::HostPort(host, port) => Uri::from_str(&format!("http://{}:{}", host, port)),
    }?;
    let channel = Channel::builder(uri);
    Ok(GatewayClient::new(channel.connect().await?))
}
