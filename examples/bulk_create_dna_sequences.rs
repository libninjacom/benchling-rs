#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_dna_sequences()
        .dna_sequences(
            vec![
                DnaSequenceBulkCreate { dna_sequence_create : DnaSequenceCreate {
                dna_sequence_base_request_for_create : DnaSequenceBaseRequestForCreate {
                dna_sequence_base_request : DnaSequenceBaseRequest { name :
                Some("your name".to_owned()), bases : Some("your bases".to_owned()),
                is_circular : Some(true), translations : Some(vec![Translation {
                sequence_feature_base : SequenceFeatureBase { color : Some("your color"
                .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField {
                value : Some("your value".to_owned()), name : Some("your name"
                .to_owned()) }]), notes : Some("your notes".to_owned()), name :
                Some("your name".to_owned()) }, end : 1, strand : 1, genetic_code :
                "your genetic code".to_owned(), regions : vec![::serde_json::json!({})],
                start : 1, amino_acids : "your amino acids".to_owned() }]), custom_fields
                : Some(CustomFields {}), folder_id : Some("your folder id".to_owned()),
                aliases : Some(vec!["your aliases".to_owned()]), primers :
                Some(vec![Primer { name : Some("your name".to_owned()), bases :
                Some("your bases".to_owned()), strand : Some(1), oligo_id :
                Some("your oligo id".to_owned()), end : Some(1), color :
                Some("your color".to_owned()), bind_position : Some(1), start : Some(1),
                overhang_length : Some(1) }]), author_ids : Some(vec!["your author ids"
                .to_owned()]), fields : Some(Fields {}), schema_id :
                Some("your schema id".to_owned()), annotations : Some(vec![DnaAnnotation
                { end : 1, strand : 1, type_ : "your type".to_owned(),
                sequence_feature_base : SequenceFeatureBase { color : Some("your color"
                .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField {
                value : Some("your value".to_owned()), name : Some("your name"
                .to_owned()) }]), notes : Some("your notes".to_owned()), name :
                Some("your name".to_owned()) }, start : 1 }]) } },
                create_entity_into_registry : CreateEntityIntoRegistry { naming_strategy
                : Some("your naming strategy".to_owned()), entity_registry_id :
                Some("your entity registry id".to_owned()), folder_id :
                Some("your folder id".to_owned()), registry_id : Some("your registry id"
                .to_owned()) } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
