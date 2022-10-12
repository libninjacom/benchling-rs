#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let box_id = "your box id";
    let response = client.get_box(box_id).send().await.unwrap();
    println!("{:#?}", response);
}
