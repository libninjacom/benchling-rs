#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_run_id = "your assay run id";
    let response = client
        .list_automation_output_processors_deprecated(assay_run_id)
        .next_token("your next token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
