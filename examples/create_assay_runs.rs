#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let assay_runs = vec![
        AssayRunCreate { fields : ::serde_json::json!({}), schema_id : "your schema id"
        .to_owned(), validation_comment : Some("your validation comment".to_owned()),
        validation_status : Some("your validation status".to_owned()), id :
        Some("your id".to_owned()), project_id : Some("your project id".to_owned()) }
    ];
    let response = client.create_assay_runs(assay_runs).send().await.unwrap();
    println!("{:#?}", response);
}
