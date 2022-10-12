#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let blob_id = "your blob id";
    let response = client.get_blob(blob_id).send().await.unwrap();
    println!("{:#?}", response);
}
