#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let molecule_id = "your molecule id";
    let response = client.get_molecule(molecule_id).send().await.unwrap();
    println!("{:#?}", response);
}
