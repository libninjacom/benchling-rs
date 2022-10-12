#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_sequence_ids = &["your dna sequence ids"];
    let response = client
        .autofill_dna_sequence_translations(dna_sequence_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
