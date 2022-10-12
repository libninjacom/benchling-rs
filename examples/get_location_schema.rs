#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let schema_id = "your schema id";
    let response = client.get_location_schema(schema_id).send().await.unwrap();
    println!("{:#?}", response);
}
