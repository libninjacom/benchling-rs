#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_task_group_id = "your workflow task group id";
    let response = client
        .get_workflow_task_group(workflow_task_group_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
