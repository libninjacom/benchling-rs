#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let custom_entity_id = "your custom entity id";
    let response = client.get_custom_entity(custom_entity_id).send().await.unwrap();
    println!("{:#?}", response);
}
