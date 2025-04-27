use log::info;
use tonic::async_trait;
use tonic::body::Body;
use tonic::codegen::http::{Request, Response};
use tonic_middleware::{Middleware, ServiceBound};

#[derive(Clone, Debug, Default)]
pub struct LoggingMiddleware {}

#[async_trait]
impl<S> Middleware<S> for LoggingMiddleware
where
    S: ServiceBound,
    S::Future: Send,
{
    async fn call(&self, req: Request<Body>, mut service: S) -> Result<Response<Body>, S::Error> {
        let uri = req.uri().to_string();

        // Log the request details
        info!("Received request on {}", uri);

        // Call the service. You can also intercept request from middleware.
        let result = service.call(req).await?;

        // Log the response details
        info!("Sending response on {}: status {}", uri, result.status());

        Ok(result)
    }
}
