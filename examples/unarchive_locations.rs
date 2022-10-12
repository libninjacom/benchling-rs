#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let location_ids = &["your location ids"];
    let response = client.unarchive_locations(location_ids).send().await.unwrap();
    println!("{:#?}", response);
}
