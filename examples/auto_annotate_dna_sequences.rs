#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_sequence_ids = &["your dna sequence ids"];
    let feature_library_ids = &["your feature library ids"];
    let response = client
        .auto_annotate_dna_sequences(dna_sequence_ids, feature_library_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
