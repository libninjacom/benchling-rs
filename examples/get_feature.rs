#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let feature_id = "your feature id";
    let response = client.get_feature(feature_id).send().await.unwrap();
    println!("{:#?}", response);
}
