#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_aa_sequences()
        .aa_sequences(
            vec![
                AaSequenceBulkCreate { aa_sequence_create : AaSequenceCreate {
                create_entity_into_registry : CreateEntityIntoRegistry {
                entity_registry_id : Some("your entity registry id".to_owned()),
                naming_strategy : Some("your naming strategy".to_owned()), registry_id :
                Some("your registry id".to_owned()), folder_id : Some("your folder id"
                .to_owned()) }, aa_sequence_base_request_for_create :
                AaSequenceBaseRequestForCreate { aa_sequence_base_request :
                AaSequenceBaseRequest { name : Some("your name".to_owned()), annotations
                : Some(vec![AaAnnotation { end : Some(1), start : Some(1), name :
                Some("your name".to_owned()), type_ : Some("your type".to_owned()), id :
                Some("your id".to_owned()), color : Some("your color".to_owned()) }]),
                author_ids : Some(vec!["your author ids".to_owned()]), custom_fields :
                Some(CustomFields {}), fields : Some(Fields {}), folder_id :
                Some("your folder id".to_owned()), amino_acids : Some("your amino acids"
                .to_owned()), schema_id : Some("your schema id".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]) } } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
