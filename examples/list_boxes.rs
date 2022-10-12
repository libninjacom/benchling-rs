#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_boxes()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .empty_positions(1)
        .empty_positions_gte(1)
        .empty_positions_gt(1)
        .empty_positions_lte(1)
        .empty_positions_lt(1)
        .empty_containers(1)
        .empty_containers_gte(1)
        .empty_containers_gt(1)
        .empty_containers_lte(1)
        .empty_containers_lt(1)
        .ancestor_storage_id("your ancestor storage id")
        .storage_contents_id("your storage contents id")
        .storage_contents_ids("your storage contents ids")
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
