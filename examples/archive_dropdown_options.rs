#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let dropdown_id = "your dropdown id";
    let response = client
        .archive_dropdown_options(dropdown_id)
        .dropdown_option_ids(&["your dropdown option ids"])
        .reason("your reason")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
