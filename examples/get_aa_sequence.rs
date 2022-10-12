#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let aa_sequence_id = "your aa sequence id";
    let response = client.get_aa_sequence(aa_sequence_id).send().await.unwrap();
    println!("{:#?}", response);
}
