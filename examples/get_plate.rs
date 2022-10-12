#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let plate_id = "your plate id";
    let response = client
        .get_plate(plate_id)
        .returning("your returning")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
