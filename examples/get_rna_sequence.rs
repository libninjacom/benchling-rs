#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let rna_sequence_id = "your rna sequence id";
    let response = client.get_rna_sequence(rna_sequence_id).send().await.unwrap();
    println!("{:#?}", response);
}
