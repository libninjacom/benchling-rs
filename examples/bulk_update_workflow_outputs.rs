#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_workflow_outputs()
        .workflow_outputs(
            vec![
                WorkflowOutputBulkUpdate { workflow_output_id : "your workflow output id"
                .to_owned(), workflow_output_write_base :
                WorkflowOutputWriteBase(::serde_json::json!({})) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
