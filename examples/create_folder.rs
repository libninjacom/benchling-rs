#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let name = "your name";
    let parent_folder_id = "your parent folder id";
    let response = client.create_folder(name, parent_folder_id).send().await.unwrap();
    println!("{:#?}", response);
}
