#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let output_processor_id = "your output processor id";
    let response = client
        .process_output_with_automation_output_processor(output_processor_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
