#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_rna_sequences()
        .rna_sequences(
            vec![
                RnaSequenceBulkCreate { rna_sequence_create : RnaSequenceCreate {
                rna_sequence_base_request_for_create : RnaSequenceBaseRequestForCreate {
                rna_sequence_base_request : RnaSequenceBaseRequest { annotations :
                Some(vec![RnaAnnotation { strand : 1, sequence_feature_base :
                SequenceFeatureBase { color : Some("your color".to_owned()),
                custom_fields : Some(vec![SequenceFeatureCustomField { value :
                Some("your value".to_owned()), name : Some("your name".to_owned()) }]),
                notes : Some("your notes".to_owned()), name : Some("your name"
                .to_owned()) }, type_ : "your type".to_owned(), end : 1, start : 1 }]),
                is_circular : Some(true), translations : Some(vec![Translation {
                sequence_feature_base : SequenceFeatureBase { color : Some("your color"
                .to_owned()), custom_fields : Some(vec![SequenceFeatureCustomField {
                value : Some("your value".to_owned()), name : Some("your name"
                .to_owned()) }]), notes : Some("your notes".to_owned()), name :
                Some("your name".to_owned()) }, end : 1, strand : 1, genetic_code :
                "your genetic code".to_owned(), regions : vec![::serde_json::json!({})],
                start : 1, amino_acids : "your amino acids".to_owned() }]), custom_fields
                : Some(CustomFields {}), name : Some("your name".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]), primers : Some(vec![Primer { name
                : Some("your name".to_owned()), bases : Some("your bases".to_owned()),
                strand : Some(1), oligo_id : Some("your oligo id".to_owned()), end :
                Some(1), color : Some("your color".to_owned()), bind_position : Some(1),
                start : Some(1), overhang_length : Some(1) }]), folder_id :
                Some("your folder id".to_owned()), fields : Some(Fields {}), author_ids :
                Some(vec!["your author ids".to_owned()]), bases : Some("your bases"
                .to_owned()), schema_id : Some("your schema id".to_owned()) } },
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
