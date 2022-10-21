#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateWorkflowTaskGroupRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateWorkflowTaskGroupRequired {
        workflow_task_group_id: "your workflow task group id",
        folder_id: "your folder id",
        name: "your name",
        watcher_ids: &["your watcher ids"],
    };
    let response = client.update_workflow_task_group(args).send().await.unwrap();
    println!("{:#?}", response);
}
