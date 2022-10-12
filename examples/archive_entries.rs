#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_ids = &["your entry ids"];
    let reason = "your reason";
    let response = client.archive_entries(entry_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
