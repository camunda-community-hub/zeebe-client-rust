pub mod auth;

use auth::{AuthInterceptor, OAuth2Config};
use generated_api::gateway_client::GatewayClient;
use thiserror::Error;
use tracing::instrument;

use tonic::{
    codegen::http::{self},
    service::Interceptor,
    transport::{self, Channel, ClientTlsConfig, Uri},
};

mod generated_api {
    tonic::include_proto!("gateway_protocol");
}

pub mod api {
    pub use super::generated_api::*;
}

#[derive(Debug)]
pub enum Connection {
    Address {
        insecure: bool,
        addr: String,
    },
    HostPort {
        insecure: bool,
        host: String,
        port: u16,
    },
}

#[derive(Debug)]
pub enum Authentication {
    Unauthenticated,
    Oauth2(OAuth2Config),
}

#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error(transparent)]
    Transport(#[from] transport::Error),
    #[error(transparent)]
    Http(#[from] http::Error),
}

struct FakeInterceptor {}

impl Interceptor for FakeInterceptor {
    fn call(&mut self, _request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        todo!()
    }
}

pub type ZeebeClient =
    GatewayClient<tonic::service::interceptor::InterceptedService<Channel, AuthInterceptor>>;

#[instrument(level = "debug")]
pub async fn connect(
    conn: Connection,
    auth: Authentication,
) -> Result<ZeebeClient, ConnectionError> {
    let insecure = match conn {
        Connection::Address { insecure, .. } => insecure,
        Connection::HostPort { insecure, .. } => insecure,
    };
    let uri = Uri::builder()
        .scheme(match insecure {
            true => "http",
            false => "https",
        })
        .authority(match &conn {
            Connection::Address { addr, .. } => addr.to_owned(),
            Connection::HostPort { host, port, .. } => format!("{}:{}", host, port),
        })
        .path_and_query("")
        .build()?;
    let interceptor = match auth {
        Authentication::Unauthenticated => AuthInterceptor::none(),
        Authentication::Oauth2(oauth_config) => AuthInterceptor::oauth2(oauth_config),
    };
    tracing::debug!("Connecting to {}", uri);
    let channel = if insecure {
        Channel::builder(uri)
    } else {
        Channel::builder(uri).tls_config(ClientTlsConfig::new())?
    };
    Ok(api::gateway_client::GatewayClient::with_interceptor(
        channel.connect().await?,
        interceptor,
    ))
}
