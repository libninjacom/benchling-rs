#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_workflow_tasks()
        .workflow_tasks(vec![WorkflowTaskBulkUpdate {}])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
