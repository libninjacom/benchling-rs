#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let location_id = "your location id";
    let response = client.get_location(location_id).send().await.unwrap();
    println!("{:#?}", response);
}
