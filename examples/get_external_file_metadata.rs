#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_id = "your entry id";
    let external_file_id = "your external file id";
    let response = client
        .get_external_file_metadata(entry_id, external_file_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
