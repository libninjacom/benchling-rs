#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let automation_output_processor_ids = &["your automation output processor ids"];
    let response = client
        .archive_automation_output_processors(automation_output_processor_ids)
        .reason("your reason")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
