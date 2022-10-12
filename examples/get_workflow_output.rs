#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let workflow_output_id = "your workflow output id";
    let response = client.get_workflow_output(workflow_output_id).send().await.unwrap();
    println!("{:#?}", response);
}
