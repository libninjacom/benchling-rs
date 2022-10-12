#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let entity_ids = &["your entity ids"];
    let naming_strategy = "your naming strategy";
    let response = client
        .register_entities(registry_id, entity_ids, naming_strategy)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
