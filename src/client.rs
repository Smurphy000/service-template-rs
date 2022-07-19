#[macro_use] extern crate log;

use example::example_client::ExampleClient;
use example::ExampleRequest;

pub mod example {
    tonic::include_proto!("example");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Creating Example Client");
    let mut client = ExampleClient::connect(
        "http://[::]:50051"
    ).await?;
    
    // Create Request as ExampleRequest
    // which is the required input for ShowExample function
    let request= tonic::Request::new(
        ExampleRequest {
            value1: "Hello, world!".to_string()
        }
    );

    info!("Calling ShowExample function");
    let response = client.show_example(request).await?;
    
    println!("Response={:?}", response);
    Ok(())
}
