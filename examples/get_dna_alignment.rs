#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_alignment_id = "your dna alignment id";
    let response = client.get_dna_alignment(dna_alignment_id).send().await.unwrap();
    println!("{:#?}", response);
}
