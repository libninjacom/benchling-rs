#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let folder_ids = &["your folder ids"];
    let reason = "your reason";
    let response = client.archive_folders(folder_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
