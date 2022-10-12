#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let oligo_ids = &["your oligo ids"];
    let reason = "your reason";
    let response = client.archive_oligos(oligo_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
