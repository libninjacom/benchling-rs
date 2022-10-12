#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_id = "your entry id";
    let response = client.get_entry(entry_id).send().await.unwrap();
    println!("{:#?}", response);
}
