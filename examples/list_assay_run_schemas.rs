#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_assay_run_schemas()
        .next_token("your next token")
        .page_size(1)
        .modified_at("your modified at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
