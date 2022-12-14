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
                DnaSequenceBulkUpdate { dna_sequence_base_request :
                DnaSequenceBaseRequest { primers : Some(vec![Primer { oligo_id :
                Some("your oligo id".to_owned()), strand : Some(1), end : Some(1), bases
                : Some("your bases".to_owned()), start : Some(1), bind_position :
                Some(1), color : Some("your color".to_owned()), overhang_length :
                Some(1), name : Some("your name".to_owned()) }]), translations :
                Some(vec![Translation { start : 1, strand : 1, amino_acids :
                "your amino acids".to_owned(), genetic_code : "your genetic code"
                .to_owned(), regions : vec![::serde_json::json!({})], end : 1,
                sequence_feature_base : SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), color :
                Some("your color".to_owned()), name : Some("your name".to_owned()), notes
                : Some("your notes".to_owned()) } }]), schema_id : Some("your schema id"
                .to_owned()), author_ids : Some(vec!["your author ids".to_owned()]),
                folder_id : Some("your folder id".to_owned()), bases : Some("your bases"
                .to_owned()), name : Some("your name".to_owned()), fields : Some(Fields
                {}), annotations : Some(vec![DnaAnnotation { strand : 1, end : 1, type_ :
                "your type".to_owned(), sequence_feature_base : SequenceFeatureBase {
                custom_fields : Some(vec![SequenceFeatureCustomField { name :
                Some("your name".to_owned()), value : Some("your value".to_owned()) }]),
                color : Some("your color".to_owned()), name : Some("your name"
                .to_owned()), notes : Some("your notes".to_owned()) }, start : 1 }]),
                aliases : Some(vec!["your aliases".to_owned()]), custom_fields :
                Some(CustomFields {}), is_circular : Some(true) }, id : "your id"
                .to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
