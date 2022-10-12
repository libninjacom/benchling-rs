#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_registries()
        .name("your name")
        .modified_at("your modified at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
