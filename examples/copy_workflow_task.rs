#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_task_id = "your workflow task id";
    let response = client.copy_workflow_task(workflow_task_id).send().await.unwrap();
    println!("{:#?}", response);
}
