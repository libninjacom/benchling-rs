#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dna_oligo_ids = &["your dna oligo ids"];
    let reason = "your reason";
    let response = client
        .archive_dna_oligos(dna_oligo_ids, reason)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
