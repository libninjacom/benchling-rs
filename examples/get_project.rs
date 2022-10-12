#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let project_id = "your project id";
    let response = client.get_project(project_id).send().await.unwrap();
    println!("{:#?}", response);
}
