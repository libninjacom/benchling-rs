#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let legacy_workflow_id = "your legacy workflow id";
    let response = client
        .list_legacy_workflow_stages(legacy_workflow_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
