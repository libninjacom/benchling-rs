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
                dna_sequence_base_request : DnaSequenceBaseRequest { schema_id :
                Some("your schema id".to_owned()), is_circular : Some(true), folder_id :
                Some("your folder id".to_owned()), primers : Some(vec![Primer { strand :
                Some(1), bases : Some("your bases".to_owned()), end : Some(1),
                bind_position : Some(1), name : Some("your name".to_owned()), start :
                Some(1), oligo_id : Some("your oligo id".to_owned()), overhang_length :
                Some(1), color : Some("your color".to_owned()) }]), fields : Some(Fields
                {}), custom_fields : Some(CustomFields {}), annotations :
                Some(vec![DnaAnnotation { sequence_feature_base : SequenceFeatureBase {
                color : Some("your color".to_owned()), name : Some("your name"
                .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField {
                value : Some("your value".to_owned()), name : Some("your name"
                .to_owned()) }]), notes : Some("your notes".to_owned()) }, type_ :
                "your type".to_owned(), end : 1, start : 1, strand : 1 }]), translations
                : Some(vec![Translation { genetic_code : "your genetic code".to_owned(),
                end : 1, sequence_feature_base : SequenceFeatureBase { color :
                Some("your color".to_owned()), name : Some("your name".to_owned()),
                custom_fields : Some(vec![SequenceFeatureCustomField { value :
                Some("your value".to_owned()), name : Some("your name".to_owned()) }]),
                notes : Some("your notes".to_owned()) }, amino_acids : "your amino acids"
                .to_owned(), regions : vec![::serde_json::json!({})], start : 1, strand :
                1 }]), aliases : Some(vec!["your aliases".to_owned()]), bases :
                Some("your bases".to_owned()), author_ids : Some(vec!["your author ids"
                .to_owned()]), name : Some("your name".to_owned()) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
