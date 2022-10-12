#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_id = "your entry id";
    let response = client
        .list_request_fulfillments(entry_id)
        .modified_at("your modified at")
        .next_token("your next token")
        .page_size(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
