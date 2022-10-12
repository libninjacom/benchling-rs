#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entity_id = "your entity id";
    let response = client.get_enitity_batches(entity_id).send().await.unwrap();
    println!("{:#?}", response);
}
