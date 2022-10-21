#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_dna_oligos()
        .dna_oligos(
            vec![
                DnaOligoCreate { oligo_create : OligoCreate {
                oligo_base_request_for_create : OligoBaseRequestForCreate {
                oligo_base_request : OligoBaseRequest { aliases :
                Some(vec!["your aliases".to_owned()]), fields : Some(Fields {}),
                folder_id : Some("your folder id".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), name : Some("your name"
                .to_owned()), custom_fields : Some(CustomFields {}), bases :
                Some("your bases".to_owned()), schema_id : Some("your schema id"
                .to_owned()) } }, create_entity_into_registry : CreateEntityIntoRegistry
                { entity_registry_id : Some("your entity registry id".to_owned()),
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
