#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let aa_sequence_ids = &["your aa sequence ids"];
    let reason = "your reason";
    let response = client
        .archive_aa_sequences(aa_sequence_ids, reason)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
