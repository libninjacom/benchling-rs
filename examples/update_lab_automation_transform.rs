#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let transform_id = "your transform id";
    let response = client
        .update_lab_automation_transform(transform_id)
        .blob_id("your blob id")
        .errors(
            vec![
                LabAutomationBenchlingAppError { message : Some("your message"
                .to_owned()) }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
