#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateWorkflowTaskGroupRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateWorkflowTaskGroupRequired {
        name: "your name",
        schema_id: "your schema id",
        folder_id: "your folder id",
        watcher_ids: &["your watcher ids"],
    };
    let response = client.create_workflow_task_group(args).send().await.unwrap();
    println!("{:#?}", response);
}
