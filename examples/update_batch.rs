#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let batch_id = "your batch id";
    let response = client
        .update_batch(batch_id)
        .default_concentration(DefaultConcentrationSummary {
            units: Some("your units".to_owned()),
            value: Some(1.0),
        })
        .fields(Fields {})
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
