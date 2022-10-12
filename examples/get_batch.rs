#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let batch_id = "your batch id";
    let response = client.get_batch(batch_id).send().await.unwrap();
    println!("{:#?}", response);
}
