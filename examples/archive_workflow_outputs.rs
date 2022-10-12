#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let reason = "your reason";
    let workflow_output_ids = &["your workflow output ids"];
    let response = client
        .archive_workflow_outputs(reason, workflow_output_ids)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
