#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_task_group_ids = &["your workflow task group ids"];
    let response = client
        .unarchive_workflow_task_groups(workflow_task_group_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
