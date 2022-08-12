use oauth2::{
    basic::{BasicClient, BasicTokenResponse},
    url::ParseError,
    AuthUrl, ClientId, ClientSecret, TokenResponse, TokenUrl,
};
use thiserror::Error;
use tonic::{metadata::MetadataValue, service::Interceptor};
use tracing::instrument;

#[derive(Debug)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_server: String,
    pub audience: String,
}

#[derive(Debug)]
pub struct OAuth2Provider {
    client: BasicClient,
    config: OAuth2Config,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token request failed")]
    TokenRequestFailed,
}

impl OAuth2Provider {
    fn from_config(config: OAuth2Config) -> Result<OAuth2Provider, ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.clone()),
            Some(ClientSecret::new(config.client_secret.clone())),
            AuthUrl::new(config.auth_server.clone())?,
            Some(TokenUrl::new(config.auth_server.clone())?),
        )
        .set_auth_type(oauth2::AuthType::RequestBody);
        Ok(OAuth2Provider { config, client })
    }

    #[instrument]
    fn get_token(&mut self) -> Result<BasicTokenResponse, AuthError> {
        let request = self
            .client
            .exchange_client_credentials()
            .add_extra_param("audience", &self.config.audience);
        tracing::debug!(request = ?request, "requesting token");
        request.request(oauth2::ureq::http_client).map_err(|e| {
            tracing::error!(error = ?e, "request to get token failed");
            AuthError::TokenRequestFailed
        })
    }
}

pub struct AuthInterceptor {
    auth: Option<OAuth2Provider>,
}

impl AuthInterceptor {
    pub fn none() -> AuthInterceptor {
        AuthInterceptor { auth: None }
    }
    pub fn oauth2(config: OAuth2Config) -> Result<AuthInterceptor, ParseError> {
        Ok(AuthInterceptor {
            auth: Some(OAuth2Provider::from_config(config)?),
        })
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        // TODO: Don't request a new token for every request. Look into tower to handle this
        // elegantly.
        if let Some(provider) = &mut self.auth {
            let token = match provider.get_token() {
                Ok(token) => token.access_token().secret().to_owned(),
                Err(e) => {
                    tracing::error!(error = ?e, "failed to get token");
                    return Err(tonic::Status::unauthenticated("failed to get token"));
                }
            };
            let header_value = format!("Bearer {}", token);
            request.metadata_mut().insert(
                "authorization",
                MetadataValue::from_str(&header_value).map_err(|_| {
                    tonic::Status::unauthenticated("token is not a valid header value")
                })?,
            );
        }
        Ok(request)
    }
}
