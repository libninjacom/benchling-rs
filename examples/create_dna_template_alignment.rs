#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaTemplateAlignmentRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaTemplateAlignmentRequired {
        name: "your name",
        algorithm: "your algorithm",
        template_sequence_id: "your template sequence id",
        files: vec![::serde_json::json!({})],
    };
    let response = client.create_dna_template_alignment(args).send().await.unwrap();
    println!("{:#?}", response);
}
