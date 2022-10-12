#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let molecule_ids = &["your molecule ids"];
    let reason = "your reason";
    let response = client.archive_molecules(molecule_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
