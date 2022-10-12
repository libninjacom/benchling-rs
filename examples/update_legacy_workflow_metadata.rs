#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let legacy_workflow_id = "your legacy workflow id";
    let response = client
        .update_legacy_workflow_metadata(legacy_workflow_id)
        .description("your description")
        .name("your name")
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
