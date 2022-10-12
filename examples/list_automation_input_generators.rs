#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_run_id = "your assay run id";
    let response = client
        .list_automation_input_generators(assay_run_id)
        .next_token("your next token")
        .modified_at("your modified at")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
