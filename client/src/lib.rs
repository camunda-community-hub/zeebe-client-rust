pub mod auth;

use auth::{AuthInterceptor, OAuth2Config};
use generated_api::gateway_client::GatewayClient;
use oauth2::url::ParseError;
use thiserror::Error;
use tracing::instrument;

use tonic::{
    codegen::http::{self},
    transport::{self, Channel, ClientTlsConfig, Uri},
};

mod generated_api {
    tonic::include_proto!("gateway_protocol");
}

pub mod api {
    pub use super::generated_api::*;
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub insecure: bool,
    pub addr: String,
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
    #[error(transparent)]
    Oauth2(#[from] ParseError),
}

pub type ZeebeClient =
    GatewayClient<tonic::service::interceptor::InterceptedService<Channel, AuthInterceptor>>;

#[instrument(level = "debug")]
pub async fn connect(
    conn: Connection,
    auth: Authentication,
) -> Result<ZeebeClient, ConnectionError> {
    let uri = Uri::builder()
        .scheme(match conn.insecure {
            true => "http",
            false => "https",
        })
        .authority(conn.addr)
        .path_and_query("")
        .build()?;
    let interceptor = match auth {
        Authentication::Unauthenticated => AuthInterceptor::none(),
        Authentication::Oauth2(oauth_config) => AuthInterceptor::oauth2(oauth_config)?,
    };
    tracing::debug!("Connecting to {}", uri);
    let channel = if conn.insecure {
        Channel::builder(uri)
    } else {
        Channel::builder(uri).tls_config(ClientTlsConfig::new())?
    };
    Ok(api::gateway_client::GatewayClient::with_interceptor(
        channel.connect().await?,
        interceptor,
    ))
}
