#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_copy_workflow_tasks()
        .workflow_task_ids(&["your workflow task ids"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
