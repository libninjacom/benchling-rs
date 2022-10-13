#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaTemplateAlignmentRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaTemplateAlignmentRequired {
        algorithm: "your algorithm",
        files: vec![::serde_json::json!({})],
        name: "your name",
        template_sequence_id: "your template sequence id",
    };
    let response = client.create_dna_template_alignment(args).send().await.unwrap();
    println!("{:#?}", response);
}
