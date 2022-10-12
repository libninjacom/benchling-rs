#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let mixture_ids = &["your mixture ids"];
    let response = client.unarchive_mixtures(mixture_ids).send().await.unwrap();
    println!("{:#?}", response);
}
