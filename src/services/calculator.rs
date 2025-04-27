tonic::include_proto!("calculator");

use tonic::{Request, Response, Status};

use calculator_server::Calculator;

#[derive(Default, Debug)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let req = request.into_inner();
        let reply = AddResponse {
            result: req.a + req.b,
        };

        Ok(Response::new(reply))
    }
}
