#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_aa_sequences()
        .aa_sequences(
            vec![
                AaSequenceBulkUpdate(::serde_json::json!({}), AaSequenceBaseRequest {
                folder_id : Some("your folder id".to_owned()), amino_acids :
                Some("your amino acids".to_owned()), custom_fields : Some(CustomFields
                {}), schema_id : Some("your schema id".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), fields : Some(Fields {}), name
                : Some("your name".to_owned()), annotations : Some(vec![AaAnnotation {
                color : Some("your color".to_owned()), id : Some("your id".to_owned()),
                end : Some(1), name : Some("your name".to_owned()), start : Some(1),
                type_ : Some("your type".to_owned()) }]), aliases :
                Some(vec!["your aliases".to_owned()]) })
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
