#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let name = "your name";
    let response = client
        .create_benchling_app(name)
        .description("your description")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
