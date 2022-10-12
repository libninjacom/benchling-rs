#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let stage_id = "your stage id";
    let response = client
        .list_legacy_workflow_stage_runs(stage_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
