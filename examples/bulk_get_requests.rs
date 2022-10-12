#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_get_requests()
        .request_ids("your request ids")
        .display_ids("your display ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
