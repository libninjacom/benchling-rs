#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let user_id = "your user id";
    let response = client.get_user(user_id).send().await.unwrap();
    println!("{:#?}", response);
}
