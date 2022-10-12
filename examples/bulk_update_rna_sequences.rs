#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_rna_sequences()
        .rna_sequences(
            vec![
                RnaSequenceBulkUpdate(::serde_json::json!({}), RnaSequenceBaseRequest {
                annotations : Some(vec![RnaAnnotation(SequenceFeatureBase { custom_fields
                : Some(vec![SequenceFeatureCustomField { value : Some("your value"
                .to_owned()), name : Some("your name".to_owned()) }]), notes :
                Some("your notes".to_owned()), color : Some("your color".to_owned()),
                name : Some("your name".to_owned()) }, ::serde_json::json!({}))]),
                author_ids : Some(vec!["your author ids".to_owned()]), fields :
                Some(Fields {}), is_circular : Some(true), name : Some("your name"
                .to_owned()), custom_fields : Some(CustomFields {}), translations :
                Some(vec![Translation(SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { value : Some("your value"
                .to_owned()), name : Some("your name".to_owned()) }]), notes :
                Some("your notes".to_owned()), color : Some("your color".to_owned()),
                name : Some("your name".to_owned()) }, ::serde_json::json!({}))]), bases
                : Some("your bases".to_owned()), folder_id : Some("your folder id"
                .to_owned()), aliases : Some(vec!["your aliases".to_owned()]), primers :
                Some(vec![Primer { start : Some(1), color : Some("your color"
                .to_owned()), oligo_id : Some("your oligo id".to_owned()), strand :
                Some(1), name : Some("your name".to_owned()), bases : Some("your bases"
                .to_owned()), bind_position : Some(1), end : Some(1), overhang_length :
                Some(1) }]), schema_id : Some("your schema id".to_owned()) })
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
