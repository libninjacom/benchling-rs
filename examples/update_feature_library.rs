#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let feature_library_id = "your feature library id";
    let response = client
        .update_feature_library(feature_library_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
