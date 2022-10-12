#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_feature_libraries()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .ids("your ids")
        .names_any_of("your names.any of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
