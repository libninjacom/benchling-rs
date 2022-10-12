#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let reason = "your reason";
    let workflow_task_ids = &["your workflow task ids"];
    let response = client
        .archive_workflow_tasks(reason, workflow_task_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
