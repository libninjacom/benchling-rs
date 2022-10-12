#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let plate_id = "your plate id";
    let response = client
        .update_plate(plate_id)
        .returning("your returning")
        .fields(Fields {})
        .name("your name")
        .parent_storage_id("your parent storage id")
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
