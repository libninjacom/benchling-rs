#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let project_ids = &["your project ids"];
    let reason = "your reason";
    let response = client.archive_projects(project_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
