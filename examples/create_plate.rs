#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client
        .create_plate(schema_id)
        .returning("your returning")
        .barcode("your barcode")
        .container_schema_id("your container schema id")
        .fields(Fields {})
        .name("your name")
        .parent_storage_id("your parent storage id")
        .project_id("your project id")
        .wells(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
