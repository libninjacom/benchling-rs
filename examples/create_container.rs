#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateContainerRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateContainerRequired {
        fields: Fields {},
        barcode: "your barcode",
        name: "your name",
        parent_storage_id: "your parent storage id",
        schema_id: "your schema id",
    };
    let response = client
        .create_container(args)
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
