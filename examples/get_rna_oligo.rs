#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let oligo_id = "your oligo id";
    let response = client.get_rna_oligo(oligo_id).send().await.unwrap();
    println!("{:#?}", response);
}
