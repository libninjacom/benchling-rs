#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_features()
        .page_size(1)
        .next_token("your next token")
        .name("your name")
        .ids("your ids")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .feature_library_id("your feature library id")
        .feature_type("your feature type")
        .match_type("your match type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
