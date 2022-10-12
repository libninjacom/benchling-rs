#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let feature_library_ids = &["your feature library ids"];
    let rna_sequence_ids = &["your rna sequence ids"];
    let response = client
        .auto_annotate_rna_sequences(feature_library_ids, rna_sequence_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
