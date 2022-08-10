use tonic::service::Interceptor;

pub struct AuthInterceptor {}

impl Interceptor for AuthInterceptor {
    fn call(&mut self, request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        Ok(request)
    }
}
