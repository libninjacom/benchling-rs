#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_batches()
        .batch_ids("your batch ids")
        .batch_names("your batch names")
        .registry_id("your registry id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
