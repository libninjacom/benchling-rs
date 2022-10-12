#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let name = "your name";
    let schema_id = "your schema id";
    let response = client
        .create_location(name, schema_id)
        .barcode("your barcode")
        .fields(Fields {})
        .parent_storage_id("your parent storage id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
