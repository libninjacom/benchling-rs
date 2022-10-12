#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let transaction_id = "your transaction id";
    let response = client
        .commit_assay_results_transaction(transaction_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
