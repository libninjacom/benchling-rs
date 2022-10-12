#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let transfers = vec![MultipleContainersTransfer {}];
    let response = client.transfer_into_containers(transfers).send().await.unwrap();
    println!("{:#?}", response);
}
