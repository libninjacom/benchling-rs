#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let reason = "your reason";
    let rna_oligo_ids = &["your rna oligo ids"];
    let response = client
        .archive_rna_oligos(reason, rna_oligo_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
