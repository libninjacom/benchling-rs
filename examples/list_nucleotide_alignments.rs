#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .list_nucleotide_alignments()
        .page_size(1)
        .next_token("your next token")
        .sort("your sort")
        .modified_at("your modified at")
        .name("your name")
        .name_includes("your name includes")
        .ids("your ids")
        .names_any_of("your names.any of")
        .names_any_of_case_sensitive("your names.any of.case sensitive")
        .sequence_ids("your sequence ids")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
