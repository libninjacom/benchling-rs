#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let box_id = "your box id";
    let response = client
        .list_box_contents(box_id)
        .page_size(1)
        .next_token("your next token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
