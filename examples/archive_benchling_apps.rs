#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let app_ids = &["your app ids"];
    let reason = "your reason";
    let response = client.archive_benchling_apps(app_ids, reason).send().await.unwrap();
    println!("{:#?}", response);
}
