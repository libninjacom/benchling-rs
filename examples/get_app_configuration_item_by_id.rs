#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let item_id = "your item id";
    let response = client
        .get_app_configuration_item_by_id(item_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
