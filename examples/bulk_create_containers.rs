#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let containers = vec![
        ContainerCreate(ContainerWriteBase(::serde_json::json!({})),
        ::serde_json::json!({}))
    ];
    let response = client.bulk_create_containers(containers).send().await.unwrap();
    println!("{:#?}", response);
}
