#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_id = "your entry id";
    let response = client
        .update_entry(entry_id)
        .author_ids("your author ids")
        .fields(Fields {})
        .folder_id("your folder id")
        .name("your name")
        .schema_id("your schema id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
