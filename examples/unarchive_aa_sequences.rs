#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let aa_sequence_ids = &["your aa sequence ids"];
    let response = client.unarchive_aa_sequences(aa_sequence_ids).send().await.unwrap();
    println!("{:#?}", response);
}
