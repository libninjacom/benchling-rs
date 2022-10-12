#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let app_ids = &["your app ids"];
    let response = client.unarchive_benchling_apps(app_ids).send().await.unwrap();
    println!("{:#?}", response);
}
