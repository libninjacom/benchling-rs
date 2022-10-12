#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let custom_entities = vec![
        CustomEntityBulkUpdate(CustomEntityBaseRequest { custom_fields :
        Some(CustomFields {}), aliases : Some(vec!["your aliases".to_owned()]), name :
        Some("your name".to_owned()), schema_id : Some("your schema id".to_owned()),
        folder_id : Some("your folder id".to_owned()), author_ids :
        Some(vec!["your author ids".to_owned()]), fields : Some(Fields {}) })
    ];
    let response = client
        .bulk_update_custom_entities(custom_entities)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
