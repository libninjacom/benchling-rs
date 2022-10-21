#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_users()
        .ids("your ids")
        .name("your name")
        .name_includes("your name includes")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .modified_at("your modified at")
        .member_of("your member of")
        .admin_of("your admin of")
        .handles("your handles")
        .password_last_changed_at("your password last changed at")
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
