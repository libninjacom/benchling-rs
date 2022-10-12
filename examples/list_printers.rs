#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let response = client
        .list_printers(registry_id)
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
