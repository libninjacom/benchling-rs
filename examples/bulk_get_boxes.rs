#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_boxes()
        .box_ids("your box ids")
        .barcodes("your barcodes")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
