#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let oligo_ids = "your oligo ids";
    let response = client.bulk_get_oligos(oligo_ids).send().await.unwrap();
    println!("{:#?}", response);
}
