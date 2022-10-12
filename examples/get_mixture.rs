#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let mixture_id = "your mixture id";
    let response = client.get_mixture(mixture_id).send().await.unwrap();
    println!("{:#?}", response);
}
