#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_workflow_outputs()
        .workflow_outputs(
            vec![
                WorkflowOutputBulkCreate(WorkflowOutputCreate(WorkflowOutputWriteBase(::serde_json::json!({})),
                ::serde_json::json!({})))
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
