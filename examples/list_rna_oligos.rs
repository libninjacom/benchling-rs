#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_rna_oligos()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .bases("your bases")
        .folder_id("your folder id")
        .mentioned_in("your mentioned in")
        .project_id("your project id")
        .registry_id("your registry id")
        .schema_id("your schema id")
        .schema_fields(SchemaFieldsQueryParam {})
        .archive_reason("your archive reason")
        .mentions("your mentions")
        .ids("your ids")
        .entity_registry_ids_any_of("your entity registry ids.any of")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .creator_ids("your creator ids")
        .author_ids_any_of("your author ids.any of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
