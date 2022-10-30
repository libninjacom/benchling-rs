#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateContainerRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateContainerRequired {
        name: "your name",
        schema_id: "your schema id",
        barcode: "your barcode",
        fields: Fields {},
        parent_storage_id: "your parent storage id",
    };
    let response = client
        .create_container(args)
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
