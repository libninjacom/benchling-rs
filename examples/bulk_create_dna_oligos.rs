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
                DnaOligoCreate(OligoCreate(OligoBaseRequestForCreate(OligoBaseRequest {
                bases : Some("your bases".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), custom_fields :
                Some(CustomFields {}), fields : Some(Fields {}), folder_id :
                Some("your folder id".to_owned()), name : Some("your name".to_owned()),
                schema_id : Some("your schema id".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]) }, ::serde_json::json!({})),
                CreateEntityIntoRegistry { entity_registry_id :
                Some("your entity registry id".to_owned()), folder_id :
                Some("your folder id".to_owned()), registry_id : Some("your registry id"
                .to_owned()), naming_strategy : Some("your naming strategy".to_owned())
                }))
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
