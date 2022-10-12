#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_ids = &["your entry ids"];
    let response = client.unarchive_entries(entry_ids).send().await.unwrap();
    println!("{:#?}", response);
}
