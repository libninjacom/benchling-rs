#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_result_ids = &["your assay result ids"];
    let reason = "your reason";
    let response = client
        .archive_assay_results(assay_result_ids, reason)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
