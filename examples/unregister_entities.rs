#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let entity_ids = &["your entity ids"];
    let folder_id = "your folder id";
    let response = client
        .unregister_entities(registry_id, entity_ids, folder_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
