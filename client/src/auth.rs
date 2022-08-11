use tonic::service::Interceptor;

#[derive(Debug)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_server: String,
    pub audience: String,
}

pub struct OAuth2Provider {
    config: OAuth2Config,
}

impl OAuth2Provider {
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
    pub fn oauth2(config: OAuth2Config) -> AuthInterceptor {
        AuthInterceptor {
            auth: Some(OAuth2Provider { config }),
        }
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
