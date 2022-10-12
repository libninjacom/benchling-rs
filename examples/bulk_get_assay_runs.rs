#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_run_ids = "your assay run ids";
    let response = client.bulk_get_assay_runs(assay_run_ids).send().await.unwrap();
    println!("{:#?}", response);
}
