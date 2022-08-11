#![feature(associated_type_bounds)]

mod auth;

use std::str::FromStr;

use auth::AuthInterceptor;
use generated_api::gateway_client::GatewayClient;
use thiserror::Error;

use tonic::{
    codegen::http::uri::InvalidUri,
    service::Interceptor,
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

struct FakeInterceptor {}

impl Interceptor for FakeInterceptor {
    fn call(&mut self, _request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        todo!()
    }
}
// Service::ResponseBody: Body<Data = Bytes> + Send + 'static,
// <Service as GrpcService<tonic::body::BoxBody>>::Future: Send;

pub type ZeebeClient =
    GatewayClient<tonic::service::interceptor::InterceptedService<Channel, AuthInterceptor>>;
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
