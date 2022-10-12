#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_id = "your workflow id";
    let response = client.list_workflow_stages(workflow_id).send().await.unwrap();
    println!("{:#?}", response);
}
