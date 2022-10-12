#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let molecule_ids = &["your molecule ids"];
    let response = client.unarchive_molecules(molecule_ids).send().await.unwrap();
    println!("{:#?}", response);
}
