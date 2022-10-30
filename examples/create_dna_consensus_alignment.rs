#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaConsensusAlignmentRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaConsensusAlignmentRequired {
        new_sequence: ::serde_json::json!({}),
        files: vec![::serde_json::json!({})],
        name: "your name",
        sequence_id: "your sequence id",
        algorithm: "your algorithm",
    };
    let response = client.create_dna_consensus_alignment(args).send().await.unwrap();
    println!("{:#?}", response);
}
