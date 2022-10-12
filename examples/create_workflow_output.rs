#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let fields = Fields {};
    let workflow_task_id = "your workflow task id";
    let response = client
        .create_workflow_output(fields, workflow_task_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
