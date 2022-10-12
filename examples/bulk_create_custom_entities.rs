#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let custom_entities = vec![
        CustomEntityBulkCreate(CustomEntityCreate(CustomEntityBaseRequestForCreate(CustomEntityBaseRequest
        { custom_fields : Some(CustomFields {}), aliases : Some(vec!["your aliases"
        .to_owned()]), name : Some("your name".to_owned()), schema_id :
        Some("your schema id".to_owned()), folder_id : Some("your folder id".to_owned()),
        author_ids : Some(vec!["your author ids".to_owned()]), fields : Some(Fields {})
        }, ::serde_json::json!({})), CreateEntityIntoRegistry { entity_registry_id :
        Some("your entity registry id".to_owned()), folder_id : Some("your folder id"
        .to_owned()), registry_id : Some("your registry id".to_owned()), naming_strategy
        : Some("your naming strategy".to_owned()) }))
    ];
    let response = client
        .bulk_create_custom_entities(custom_entities)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
