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
                dna_sequence_base_request : DnaSequenceBaseRequest { annotations :
                Some(vec![DnaAnnotation { end : 1, strand : 1, sequence_feature_base :
                SequenceFeatureBase { color : Some("your color".to_owned()), name :
                Some("your name".to_owned()), custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), notes :
                Some("your notes".to_owned()) }, start : 1, type_ : "your type"
                .to_owned() }]), bases : Some("your bases".to_owned()), primers :
                Some(vec![Primer { end : Some(1), overhang_length : Some(1), oligo_id :
                Some("your oligo id".to_owned()), bind_position : Some(1), color :
                Some("your color".to_owned()), bases : Some("your bases".to_owned()),
                name : Some("your name".to_owned()), start : Some(1), strand : Some(1)
                }]), aliases : Some(vec!["your aliases".to_owned()]), schema_id :
                Some("your schema id".to_owned()), translations : Some(vec![Translation {
                amino_acids : "your amino acids".to_owned(), sequence_feature_base :
                SequenceFeatureBase { color : Some("your color".to_owned()), name :
                Some("your name".to_owned()), custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), notes :
                Some("your notes".to_owned()) }, end : 1, genetic_code :
                "your genetic code".to_owned(), regions : vec![::serde_json::json!({})],
                strand : 1, start : 1 }]), fields : Some(Fields {}), custom_fields :
                Some(CustomFields {}), folder_id : Some("your folder id".to_owned()),
                author_ids : Some(vec!["your author ids".to_owned()]), is_circular :
                Some(true), name : Some("your name".to_owned()) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
