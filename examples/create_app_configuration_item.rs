#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client.create_app_configuration_item().send().await.unwrap();
    println!("{:#?}", response);
}
