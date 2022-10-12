#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let request_id = "your request id";
    let tasks = vec![RequestTasksBulkCreate { schema_id : "your schema id".to_owned() }];
    let response = client
        .bulk_create_request_tasks(request_id, tasks)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
