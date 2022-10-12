#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let request_id = "your request id";
    let tasks = vec![RequestTaskBase { id : "your id".to_owned() }];
    let response = client
        .bulk_update_request_tasks(request_id, tasks)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
