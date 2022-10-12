#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_projects()
        .next_token("your next token")
        .page_size(1)
        .sort("your sort")
        .archive_reason("your archive reason")
        .ids("your ids")
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
