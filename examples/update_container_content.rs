#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_id = "your container id";
    let containable_id = "your containable id";
    let concentration = Measurement {
        units: Some("your units".to_owned()),
        value: Some(1.0),
    };
    let response = client
        .update_container_content(container_id, containable_id, concentration)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
