#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_result_id = "your assay result id";
    let response = client.get_assay_result(assay_result_id).send().await.unwrap();
    println!("{:#?}", response);
}
