#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let box_id = "your box id";
    let response = client
        .update_box(box_id)
        .fields(Fields {})
        .name("your name")
        .parent_storage_id("your parent storage id")
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
