#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let reason = "your reason";
    let rna_sequence_ids = &["your rna sequence ids"];
    let response = client
        .archive_rna_sequences(reason, rna_sequence_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
