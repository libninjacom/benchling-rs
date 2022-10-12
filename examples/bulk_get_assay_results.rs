#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_result_ids = "your assay result ids";
    let response = client.bulk_get_assay_results(assay_result_ids).send().await.unwrap();
    println!("{:#?}", response);
}
