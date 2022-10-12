#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_run_id = "your assay run id";
    let response = client.get_assay_run(assay_run_id).send().await.unwrap();
    println!("{:#?}", response);
}
