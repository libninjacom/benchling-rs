#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_entries()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .project_id("your project id")
        .archive_reason("your archive reason")
        .review_status("your review status")
        .mentioned_in("your mentioned in")
        .mentions("your mentions")
        .ids("your ids")
        .schema_id("your schema id")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .assigned_reviewer_ids_any_of("your assigned reviewer ids.any of")
        .creator_ids("your creator ids")
        .author_ids_any_of("your author ids.any of")
        .display_ids("your display ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
