use oauth2::{basic::BasicClient, TokenUrl, ClientSecret, AuthUrl, ClientId, url::ParseError};
use tonic::service::Interceptor;

#[derive(Debug)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_server: String,
    pub audience: String,
}

pub struct OAuth2Provider {
    client: oauth2::basic::BasicClient,
    config: OAuth2Config,
}

impl OAuth2Provider {
    fn from_config(config: OAuth2Config) -> Result<OAuth2Provider, ParseError> {
        let client = BasicClient::new(
            ClientId::new(config.client_id.clone()),
            Some(ClientSecret::new(config.client_secret.clone())),
            AuthUrl::new(config.auth_server.clone())?,
            Some(TokenUrl::new(config.auth_server.clone())?),
        );
        Ok(OAuth2Provider {config, client})
    }
    fn get_token(&mut self) -> String {
        "fake token".to_owned()
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
        if let Some(provider) = &mut self.auth {
            let value = provider.get_token().try_into().map_err(|_err| {
                tonic::Status::failed_precondition("Couldn't build authorization header")
            })?;
            request.metadata_mut().insert("authorization", value);
        }
        Ok(request)
    }
}
