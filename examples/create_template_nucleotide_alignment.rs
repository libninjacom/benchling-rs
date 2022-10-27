#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateTemplateNucleotideAlignmentRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateTemplateNucleotideAlignmentRequired {
        files: vec![::serde_json::json!({})],
        algorithm: "your algorithm",
        name: "your name",
        template_sequence_id: "your template sequence id",
    };
    let response = client
        .create_template_nucleotide_alignment(args)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
