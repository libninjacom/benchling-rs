#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let request_id = "your request id";
    let response = client.get_request_response(request_id).send().await.unwrap();
    println!("{:#?}", response);
}
