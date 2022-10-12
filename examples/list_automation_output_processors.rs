#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_automation_output_processors()
        .assay_run_id("your assay run id")
        .automation_file_config_name("your automation file config name")
        .archive_reason("your archive reason")
        .modified_at("your modified at")
        .next_token("your next token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
