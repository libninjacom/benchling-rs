#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_plate_schemas()
        .next_token("your next token")
        .page_size(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
