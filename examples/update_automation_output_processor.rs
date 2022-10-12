#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let output_processor_id = "your output processor id";
    let file_id = "your file id";
    let response = client
        .update_automation_output_processor(output_processor_id, file_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
