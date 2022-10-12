#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let box_ids = &["your box ids"];
    let response = client.unarchive_boxes(box_ids).send().await.unwrap();
    println!("{:#?}", response);
}
