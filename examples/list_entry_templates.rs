#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_entry_templates()
        .page_size(1)
        .next_token("your next token")
        .modified_at("your modified at")
        .name("your name")
        .template_collection_id("your template collection id")
        .ids("your ids")
        .schema_id("your schema id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
