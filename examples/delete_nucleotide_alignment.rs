#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let alignment_id = "your alignment id";
    let response = client
        .delete_nucleotide_alignment(alignment_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
