#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerBulkUpdateItem(ContainerWriteBase(::serde_json::json!({})),
        ::serde_json::json!({}))
    ];
    let response = client.bulk_update_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}
