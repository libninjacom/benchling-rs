#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let entity_registry_ids = "your entity registry ids";
    let response = client
        .bulk_get_registered_entities(registry_id, entity_registry_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
