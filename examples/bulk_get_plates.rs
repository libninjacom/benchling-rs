#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_plates()
        .plate_ids("your plate ids")
        .barcodes("your barcodes")
        .returning("your returning")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
