#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .create_batch()
        .default_concentration(DefaultConcentrationSummary {
            units: Some("your units".to_owned()),
            value: Some(1.0),
        })
        .entity_id("your entity id")
        .fields(Fields {})
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
