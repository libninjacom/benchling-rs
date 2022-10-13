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
                DnaSequenceBulkUpdate { id : "your id".to_owned(),
                dna_sequence_base_request : DnaSequenceBaseRequest { is_circular :
                Some(true), fields : Some(Fields {}), bases : Some("your bases"
                .to_owned()), annotations : Some(vec![DnaAnnotation { start : 1,
                sequence_feature_base : SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), color :
                Some("your color".to_owned()), name : Some("your name".to_owned()), notes
                : Some("your notes".to_owned()) }, strand : 1, type_ : "your type"
                .to_owned(), end : 1 }]), aliases : Some(vec!["your aliases"
                .to_owned()]), primers : Some(vec![Primer { bases : Some("your bases"
                .to_owned()), color : Some("your color".to_owned()), end : Some(1), name
                : Some("your name".to_owned()), oligo_id : Some("your oligo id"
                .to_owned()), overhang_length : Some(1), bind_position : Some(1), start :
                Some(1), strand : Some(1) }]), folder_id : Some("your folder id"
                .to_owned()), translations : Some(vec![Translation { strand : 1, regions
                : vec![::serde_json::json!({})], start : 1, end : 1,
                sequence_feature_base : SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), color :
                Some("your color".to_owned()), name : Some("your name".to_owned()), notes
                : Some("your notes".to_owned()) }, genetic_code : "your genetic code"
                .to_owned(), amino_acids : "your amino acids".to_owned() }]), schema_id :
                Some("your schema id".to_owned()), custom_fields : Some(CustomFields {}),
                author_ids : Some(vec!["your author ids".to_owned()]), name :
                Some("your name".to_owned()) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
