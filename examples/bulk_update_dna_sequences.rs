#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_dna_sequences()
        .dna_sequences(
            vec![
                DnaSequenceBulkUpdate(::serde_json::json!({}), DnaSequenceBaseRequest {
                name : Some("your name".to_owned()), schema_id : Some("your schema id"
                .to_owned()), bases : Some("your bases".to_owned()), folder_id :
                Some("your folder id".to_owned()), is_circular : Some(true), annotations
                : Some(vec![DnaAnnotation(SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { value : Some("your value"
                .to_owned()), name : Some("your name".to_owned()) }]), notes :
                Some("your notes".to_owned()), color : Some("your color".to_owned()),
                name : Some("your name".to_owned()) }, ::serde_json::json!({}))]),
                primers : Some(vec![Primer { start : Some(1), color : Some("your color"
                .to_owned()), oligo_id : Some("your oligo id".to_owned()), strand :
                Some(1), name : Some("your name".to_owned()), bases : Some("your bases"
                .to_owned()), bind_position : Some(1), end : Some(1), overhang_length :
                Some(1) }]), translations : Some(vec![Translation(SequenceFeatureBase {
                custom_fields : Some(vec![SequenceFeatureCustomField { value :
                Some("your value".to_owned()), name : Some("your name".to_owned()) }]),
                notes : Some("your notes".to_owned()), color : Some("your color"
                .to_owned()), name : Some("your name".to_owned()) },
                ::serde_json::json!({}))]), custom_fields : Some(CustomFields {}),
                aliases : Some(vec!["your aliases".to_owned()]), fields : Some(Fields
                {}), author_ids : Some(vec!["your author ids".to_owned()]) })
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
