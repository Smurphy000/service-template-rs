#[macro_use] extern crate log;

use tonic::{transport::Server, Request, Response, Status};

use example::example_server::{Example,ExampleServer};
use example::{ExampleRequest, ExampleResponse};

pub mod example {
    tonic::include_proto!("example");
}


#[derive(Debug, Default)]
pub struct ExampleService {}

#[tonic::async_trait]
impl Example for ExampleService {
    async fn show_example(&self, request: Request<ExampleRequest>) -> Result<Response<ExampleResponse>, Status> {
        info!("Running ShowExample");
        let req = request.into_inner();
        let example_response = ExampleResponse {
            value1: req.value1
        };

        Ok(Response::new(example_response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "[::]:50051".parse()?;
    let example_service = ExampleService::default();
    
    info!("Starting ExampleService");
    Server::builder()
        .add_service(ExampleServer::new(example_service))
        .serve(addr)
        .await?;

    Ok(())
}
