#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_app_configuration_items()
        .next_token("your next token")
        .page_size(1)
        .modified_at("your modified at")
        .app_id("your app id")
        .ids("your ids")
        .sort("your sort")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
