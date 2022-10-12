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
                WorkflowTaskBulkCreate(WorkflowTaskCreate(WorkflowTaskWriteBase(::serde_json::json!({})),
                ::serde_json::json!({})))
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
