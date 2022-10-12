#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client
        .list_assay_runs(schema_id)
        .min_created_time(1)
        .max_created_time(1)
        .next_token("your next token")
        .page_size(1)
        .ids("your ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
