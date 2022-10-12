#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let destination_container_id = "your destination container id";
    let response = client
        .transfer_into_container(destination_container_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
