#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_workflow_tasks()
        .workflow_tasks(
            vec![
                WorkflowTaskBulkCreate { workflow_task_create : WorkflowTaskCreate {
                workflow_task_write_base :
                WorkflowTaskWriteBase(::serde_json::json!({})), workflow_task_group_id :
                "your workflow task group id".to_owned() } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
