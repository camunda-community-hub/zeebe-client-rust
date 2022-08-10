mod auth;
use std::str::FromStr;

use auth::AuthInterceptor;
use thiserror::Error;
use tonic::{
    codegen::{http::uri::InvalidUri, InterceptedService},
    transport::{self, Channel, Uri},
};

mod generated_api {
    tonic::include_proto!("gateway_protocol");
}

pub mod api {
    pub use super::generated_api::*;
}

#[derive(Debug)]
pub enum Protocol {
    HTTPS,
    HTTP,
}
pub enum Connection {
    Address(String),
    HostPort(Protocol, String, u16),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    Transport(#[from] transport::Error),
    #[error(transparent)]
    Uri(#[from] InvalidUri),
}

pub type ZeebeClient =
    api::gateway_client::GatewayClient<InterceptedService<Channel, AuthInterceptor>>;

pub async fn connect(conn: Connection) -> Result<ZeebeClient, ConnectionError> {
    let uri = match conn {
        Connection::Address(addr) => Uri::from_str(&addr),
        Connection::HostPort(proto, host, port) => {
            Uri::from_str(&format!("{:?}://{}:{}", proto, host, port))
        }
    }?;
    let channel = Channel::builder(uri);
    Ok(api::gateway_client::GatewayClient::with_interceptor(
        channel.connect().await?,
        AuthInterceptor {},
    ))
}
