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
                WorkflowOutputBulkCreate { workflow_output_create : WorkflowOutputCreate
                { workflow_output_write_base :
                WorkflowOutputWriteBase(::serde_json::json!({})), workflow_task_id :
                "your workflow task id".to_owned() } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
