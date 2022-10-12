#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let task_id = "your task id";
    let response = client.get_task(task_id).send().await.unwrap();
    println!("{:#?}", response);
}
