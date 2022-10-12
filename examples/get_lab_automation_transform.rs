#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let transform_id = "your transform id";
    let response = client
        .get_lab_automation_transform(transform_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
