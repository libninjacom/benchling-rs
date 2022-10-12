#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_oligo_ids = &["your dna oligo ids"];
    let response = client.unarchive_dna_oligos(dna_oligo_ids).send().await.unwrap();
    println!("{:#?}", response);
}
