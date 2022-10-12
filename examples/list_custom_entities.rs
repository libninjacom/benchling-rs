#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_custom_entities()
        .next_token("your next token")
        .page_size(1)
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .returning("your returning")
        .name_includes("your name includes")
        .folder_id("your folder id")
        .mentioned_in("your mentioned in")
        .project_id("your project id")
        .registry_id("your registry id")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .archive_reason("your archive reason")
        .mentions("your mentions")
        .ids("your ids")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .entity_registry_ids_any_of("your entity registry ids.any of")
        .creator_ids("your creator ids")
        .author_ids_any_of("your author ids.any of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
