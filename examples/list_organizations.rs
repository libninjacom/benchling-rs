#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_organizations()
        .ids("your ids")
        .name("your name")
        .name_includes("your name includes")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .modified_at("your modified at")
        .has_members("your has members")
        .has_admins("your has admins")
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
