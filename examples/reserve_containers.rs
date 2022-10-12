#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assignee_id = "your assignee id";
    let container_ids = &["your container ids"];
    let response = client
        .reserve_containers(assignee_id, container_ids)
        .comment("your comment")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
