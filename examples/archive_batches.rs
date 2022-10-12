#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let batch_ids = &["your batch ids"];
    let reason = "your reason";
    let response = client.archive_batches(batch_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
