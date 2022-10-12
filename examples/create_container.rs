#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateContainerRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateContainerRequired {
        parent_storage_id: "your parent storage id",
        barcode: "your barcode",
        name: "your name",
        schema_id: "your schema id",
        fields: Fields {},
    };
    let response = client
        .create_container(args)
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}