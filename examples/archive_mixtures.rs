#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let mixture_ids = &["your mixture ids"];
    let reason = "your reason";
    let response = client.archive_mixtures(mixture_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
