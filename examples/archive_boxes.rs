#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let box_ids = &["your box ids"];
    let reason = "your reason";
    let response = client
        .archive_boxes(box_ids, reason)
        .should_remove_barcodes(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
