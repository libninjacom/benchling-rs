#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client
        .create_box(schema_id)
        .barcode("your barcode")
        .fields(Fields {})
        .name("your name")
        .parent_storage_id("your parent storage id")
        .project_id("your project id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
