#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_folders()
        .next_token("your next token")
        .page_size(1)
        .sort("your sort")
        .archive_reason("your archive reason")
        .name_includes("your name includes")
        .parent_folder_id("your parent folder id")
        .project_id("your project id")
        .ids("your ids")
        .name("your name")
        .section("your section")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
