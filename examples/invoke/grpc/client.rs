use std::{thread, time::Duration};

use hello_world::{HelloReply, HelloRequest};
use prost::Message;

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

type DaprClient = dapr::Client<dapr::client::TonicClient>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sleep to allow for the server to become available
    thread::sleep(Duration::from_secs(5));

    // Set the Dapr address
    let address = "https://127.0.0.1".to_string();

    let mut client = DaprClient::connect(address, None).await?;

    let request = HelloRequest {
        name: "Test".to_string(),
    };
    let data = request.encode_to_vec();
    let data = prost_types::Any {
        type_url: "".to_string(),
        value: data,
    };

    let response = client
        .invoke_service("invoke-grpc-server", "say_hello", Some(data))
        .await
        .unwrap();

    if let Some(any) = &response.data {
        let data = &any.value;
        let resp = HelloReply::decode(&data[..]).unwrap();
        println!("Message: {:#?}", &resp.message);
    };

    println!("Response: {:#?}", response);

    Ok(())
}
