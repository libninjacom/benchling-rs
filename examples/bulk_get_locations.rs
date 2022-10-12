#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_locations()
        .location_ids("your location ids")
        .barcodes("your barcodes")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
