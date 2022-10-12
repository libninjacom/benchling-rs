#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let stage_run_id = "your stage run id";
    let response = client
        .list_legacy_workflow_stage_run_registered_samples(stage_run_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
