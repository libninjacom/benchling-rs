#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_ids = &["your container ids"];
    let reason = "your reason";
    let response = client
        .archive_containers(container_ids, reason)
        .should_remove_barcodes(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
