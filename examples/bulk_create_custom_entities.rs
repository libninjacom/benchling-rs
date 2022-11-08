#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let custom_entities = vec![
        CustomEntityBulkCreate { custom_entity_create : CustomEntityCreate {
        create_entity_into_registry : CreateEntityIntoRegistry { folder_id :
        Some("your folder id".to_owned()), naming_strategy : Some("your naming strategy"
        .to_owned()), entity_registry_id : Some("your entity registry id".to_owned()),
        registry_id : Some("your registry id".to_owned()) },
        custom_entity_base_request_for_create : CustomEntityBaseRequestForCreate {
        custom_entity_base_request : CustomEntityBaseRequest { schema_id :
        Some("your schema id".to_owned()), author_ids : Some(vec!["your author ids"
        .to_owned()]), fields : Some(Fields {}), aliases : Some(vec!["your aliases"
        .to_owned()]), custom_fields : Some(CustomFields {}), folder_id :
        Some("your folder id".to_owned()), name : Some("your name".to_owned()) } } } }
    ];
    let response = client
        .bulk_create_custom_entities(custom_entities)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
