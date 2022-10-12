#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let app_id = "your app id";
    let response = client.get_benchling_app_by_id(app_id).send().await.unwrap();
    println!("{:#?}", response);
}
