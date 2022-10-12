#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let folder_id = "your folder id";
    let response = client.get_folder(folder_id).send().await.unwrap();
    println!("{:#?}", response);
}
