#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_run_id = "your assay run id";
    let automation_file_config_name = "your automation file config name";
    let file_id = "your file id";
    let response = client
        .create_automation_output_processor(
            assay_run_id,
            automation_file_config_name,
            file_id,
        )
        .complete_with_errors(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
