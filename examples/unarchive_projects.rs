#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let project_ids = &["your project ids"];
    let response = client.unarchive_projects(project_ids).send().await.unwrap();
    println!("{:#?}", response);
}
