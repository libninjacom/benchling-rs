#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client.create_assay_results_transaction().send().await.unwrap();
    println!("{:#?}", response);
}
