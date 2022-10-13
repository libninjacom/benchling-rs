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
                rna_sequence_base_request : RnaSequenceBaseRequest { name :
                Some("your name".to_owned()), annotations : Some(vec![RnaAnnotation { end
                : 1, type_ : "your type".to_owned(), strand : 1, sequence_feature_base :
                SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), color :
                Some("your color".to_owned()), name : Some("your name".to_owned()), notes
                : Some("your notes".to_owned()) }, start : 1 }]), bases :
                Some("your bases".to_owned()), custom_fields : Some(CustomFields {}),
                is_circular : Some(true), aliases : Some(vec!["your aliases"
                .to_owned()]), author_ids : Some(vec!["your author ids".to_owned()]),
                fields : Some(Fields {}), folder_id : Some("your folder id".to_owned()),
                primers : Some(vec![Primer { bases : Some("your bases".to_owned()), color
                : Some("your color".to_owned()), end : Some(1), name : Some("your name"
                .to_owned()), oligo_id : Some("your oligo id".to_owned()),
                overhang_length : Some(1), bind_position : Some(1), start : Some(1),
                strand : Some(1) }]), schema_id : Some("your schema id".to_owned()),
                translations : Some(vec![Translation { strand : 1, regions :
                vec![::serde_json::json!({})], start : 1, end : 1, sequence_feature_base
                : SequenceFeatureBase { custom_fields :
                Some(vec![SequenceFeatureCustomField { name : Some("your name"
                .to_owned()), value : Some("your value".to_owned()) }]), color :
                Some("your color".to_owned()), name : Some("your name".to_owned()), notes
                : Some("your notes".to_owned()) }, genetic_code : "your genetic code"
                .to_owned(), amino_acids : "your amino acids".to_owned() }]) } },
                create_entity_into_registry : CreateEntityIntoRegistry {
                entity_registry_id : Some("your entity registry id".to_owned()),
                naming_strategy : Some("your naming strategy".to_owned()), folder_id :
                Some("your folder id".to_owned()), registry_id : Some("your registry id"
                .to_owned()) } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
