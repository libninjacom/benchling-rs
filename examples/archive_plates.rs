#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let plate_ids = &["your plate ids"];
    let reason = "your reason";
    let should_remove_barcodes = true;
    let response = client
        .archive_plates(plate_ids, reason, should_remove_barcodes)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
