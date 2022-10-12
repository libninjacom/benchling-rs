#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client
        .list_requests(schema_id)
        .request_status("your request status")
        .min_created_time(1)
        .max_created_time(1)
        .next_token("your next token")
        .page_size(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
