#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_benchling_apps()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .ids("your ids")
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .creator_ids("your creator ids")
        .member_of("your member of")
        .admin_of("your admin of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
