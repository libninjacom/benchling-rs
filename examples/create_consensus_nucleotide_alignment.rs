#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateConsensusNucleotideAlignmentRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateConsensusNucleotideAlignmentRequired {
        files: vec![::serde_json::json!({})],
        new_sequence: ::serde_json::json!({}),
        algorithm: "your algorithm",
        name: "your name",
        sequence_id: "your sequence id",
    };
    let response = client
        .create_consensus_nucleotide_alignment(args)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
