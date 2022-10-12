#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let batch_ids = &["your batch ids"];
    let response = client.unarchive_batches(batch_ids).send().await.unwrap();
    println!("{:#?}", response);
}
