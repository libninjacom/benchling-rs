#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_blobs()
        .blob_ids("your blob ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
