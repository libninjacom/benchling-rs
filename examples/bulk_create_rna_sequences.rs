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
                rna_sequence_base_request : RnaSequenceBaseRequest { schema_id :
                Some("your schema id".to_owned()), custom_fields : Some(CustomFields {}),
                aliases : Some(vec!["your aliases".to_owned()]), folder_id :
                Some("your folder id".to_owned()), is_circular : Some(true), translations
                : Some(vec![Translation { genetic_code : "your genetic code".to_owned(),
                end : 1, sequence_feature_base : SequenceFeatureBase { color :
                Some("your color".to_owned()), name : Some("your name".to_owned()),
                custom_fields : Some(vec![SequenceFeatureCustomField { value :
                Some("your value".to_owned()), name : Some("your name".to_owned()) }]),
                notes : Some("your notes".to_owned()) }, amino_acids : "your amino acids"
                .to_owned(), regions : vec![::serde_json::json!({})], start : 1, strand :
                1 }]), bases : Some("your bases".to_owned()), name : Some("your name"
                .to_owned()), fields : Some(Fields {}), annotations :
                Some(vec![RnaAnnotation { strand : 1, type_ : "your type".to_owned(),
                sequence_feature_base : SequenceFeatureBase { color : Some("your color"
                .to_owned()), name : Some("your name".to_owned()), custom_fields :
                Some(vec![SequenceFeatureCustomField { value : Some("your value"
                .to_owned()), name : Some("your name".to_owned()) }]), notes :
                Some("your notes".to_owned()) }, start : 1, end : 1 }]), author_ids :
                Some(vec!["your author ids".to_owned()]), primers : Some(vec![Primer {
                strand : Some(1), bases : Some("your bases".to_owned()), end : Some(1),
                bind_position : Some(1), name : Some("your name".to_owned()), start :
                Some(1), oligo_id : Some("your oligo id".to_owned()), overhang_length :
                Some(1), color : Some("your color".to_owned()) }]) } },
                create_entity_into_registry : CreateEntityIntoRegistry {
                entity_registry_id : Some("your entity registry id".to_owned()),
                naming_strategy : Some("your naming strategy".to_owned()), registry_id :
                Some("your registry id".to_owned()), folder_id : Some("your folder id"
                .to_owned()) } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
