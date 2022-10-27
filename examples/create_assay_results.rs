#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_results = vec![
        AssayResultCreate { fields : ::serde_json::json!({}), schema_id :
        "your schema id".to_owned(), id : Some("your id".to_owned()), field_validation :
        Some(::serde_json::json!({})), project_id : Some("your project id".to_owned()) }
    ];
    let response = client.create_assay_results(assay_results).send().await.unwrap();
    println!("{:#?}", response);
}
