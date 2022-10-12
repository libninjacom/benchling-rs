#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_id = "your container id";
    let response = client.list_container_contents(container_id).send().await.unwrap();
    println!("{:#?}", response);
}
