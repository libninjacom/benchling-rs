#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let location_id = "your location id";
    let response = client
        .update_location(location_id)
        .fields(Fields {})
        .name("your name")
        .parent_storage_id("your parent storage id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
