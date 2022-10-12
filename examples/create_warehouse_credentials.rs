#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let expires_in = 1;
    let response = client.create_warehouse_credentials(expires_in).send().await.unwrap();
    println!("{:#?}", response);
}
