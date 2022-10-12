#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let response = client
        .list_entity_schemas_by_registry(registry_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
