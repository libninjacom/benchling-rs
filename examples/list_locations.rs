#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_locations()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .ancestor_storage_id("your ancestor storage id")
        .archive_reason("your archive reason")
        .ids("your ids")
        .barcodes("your barcodes")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .creator_ids("your creator ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
