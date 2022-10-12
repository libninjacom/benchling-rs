#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let request_fulfillment_id = "your request fulfillment id";
    let response = client
        .get_request_fulfillment(request_fulfillment_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
