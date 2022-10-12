#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_events()
        .page_size(1)
        .next_token("your next token")
        .created_at_gte("your created at.gte")
        .starting_after("your starting after")
        .event_types("your event types")
        .poll(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
