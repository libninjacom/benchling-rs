#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_ids = &["your container ids"];
    let response = client
        .checkin_containers(container_ids)
        .comments("your comments")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
