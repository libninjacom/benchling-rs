#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let registry_id = "your registry id";
    let barcodes = &["your barcodes"];
    let response = client.validate_barcodes(registry_id, barcodes).send().await.unwrap();
    println!("{:#?}", response);
}
