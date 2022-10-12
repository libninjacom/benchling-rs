#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let plate_ids = &["your plate ids"];
    let response = client.unarchive_plates(plate_ids).send().await.unwrap();
    println!("{:#?}", response);
}
