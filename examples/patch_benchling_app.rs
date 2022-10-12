#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let app_id = "your app id";
    let response = client
        .patch_benchling_app(app_id)
        .description("your description")
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
