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
                DnaOligoCreate { oligo_create : OligoCreate { create_entity_into_registry
                : CreateEntityIntoRegistry { naming_strategy :
                Some("your naming strategy".to_owned()), registry_id :
                Some("your registry id".to_owned()), folder_id : Some("your folder id"
                .to_owned()), entity_registry_id : Some("your entity registry id"
                .to_owned()) }, oligo_base_request_for_create : OligoBaseRequestForCreate
                { oligo_base_request : OligoBaseRequest { schema_id :
                Some("your schema id".to_owned()), bases : Some("your bases".to_owned()),
                folder_id : Some("your folder id".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), aliases :
                Some(vec!["your aliases".to_owned()]), fields : Some(Fields {}),
                custom_fields : Some(CustomFields {}), name : Some("your name"
                .to_owned()) } } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
