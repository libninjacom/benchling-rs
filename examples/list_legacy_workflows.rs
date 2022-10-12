#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client.list_legacy_workflows().send().await.unwrap();
    println!("{:#?}", response);
}
