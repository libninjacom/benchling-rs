#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let transaction_id = "your transaction id";
    let assay_results = vec![
        AssayResultCreate { schema_id : "your schema id".to_owned(), field_validation :
        Some(::serde_json::json!({})), fields : ::serde_json::json!({}), id :
        Some("your id".to_owned()), project_id : Some("your project id".to_owned()) }
    ];
    let response = client
        .create_assay_results_in_transaction(transaction_id, assay_results)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
