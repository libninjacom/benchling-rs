#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_id = "your workflow id";
    let response = client
        .update_workflow_metadata(workflow_id)
        .description("your description")
        .name("your name")
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
