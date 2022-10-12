#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let id = "your id";
    let response = client.export_item(id).send().await.unwrap();
    println!("{:#?}", response);
}
