#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_task_ids = &["your workflow task ids"];
    let response = client
        .unarchive_workflow_tasks(workflow_task_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
