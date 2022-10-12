#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let folder_ids = &["your folder ids"];
    let response = client.unarchive_folders(folder_ids).send().await.unwrap();
    println!("{:#?}", response);
}
