use crate::hello_world::HelloReply;
use std::time::Duration;

use prost::Message;

pub mod hello_world {
    include!("../protos/helloworld.rs");
}

type DaprClient = dapr::Client<dapr::client::TonicClient>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sleep to allow for the server to become available
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Set the Dapr address
    let address = "https://127.0.0.1".to_string();

    let mut client = DaprClient::connect(address).await?;

    let request = hello_world::HelloRequest {
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

    println!("Response: {response:#?}");

    Ok(())
}
