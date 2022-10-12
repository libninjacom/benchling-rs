#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_batches()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .archive_reason("your archive reason")
        .ids("your ids")
        .creator_ids("your creator ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
