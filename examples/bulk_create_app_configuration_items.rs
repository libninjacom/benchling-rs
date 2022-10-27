#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let app_configuration_items = vec![AppConfigItemCreate {}];
    let response = client
        .bulk_create_app_configuration_items(app_configuration_items)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
