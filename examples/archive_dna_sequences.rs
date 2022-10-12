#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_sequence_ids = &["your dna sequence ids"];
    let reason = "your reason";
    let response = client
        .archive_dna_sequences(dna_sequence_ids, reason)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
