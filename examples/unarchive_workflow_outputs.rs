#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_output_ids = &["your workflow output ids"];
    let response = client
        .unarchive_workflow_outputs(workflow_output_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
