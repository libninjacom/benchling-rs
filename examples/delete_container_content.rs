#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_id = "your container id";
    let containable_id = "your containable id";
    let response = client
        .delete_container_content(container_id, containable_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
