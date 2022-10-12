#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dropdown_id = "your dropdown id";
    let response = client.get_dropdown(dropdown_id).send().await.unwrap();
    println!("{:#?}", response);
}
