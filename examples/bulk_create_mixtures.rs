#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let mixtures = vec![
        MixtureCreate { mixture_update : MixtureUpdate { folder_id :
        Some("your folder id".to_owned()), entity_registry_id :
        Some("your entity registry id".to_owned()), amount : Some("your amount"
        .to_owned()), ingredients : Some(vec![IngredientWriteParams { units :
        Some("your units".to_owned()), amount : Some("your amount".to_owned()),
        component_lot_text : Some("your component lot text".to_owned()), notes :
        Some("your notes".to_owned()), component_entity_id : "your component entity id"
        .to_owned(), catalog_identifier : Some("your catalog identifier".to_owned()),
        component_lot_container_id : Some("your component lot container id".to_owned()),
        component_lot_entity_id : Some("your component lot entity id".to_owned()) }]),
        schema_id : Some("your schema id".to_owned()), fields : Some(Fields {}),
        author_ids : Some(vec!["your author ids".to_owned()]), name : Some("your name"
        .to_owned()), custom_fields : Some(CustomFields {}), units : Some("your units"
        .to_owned()), aliases : Some(vec!["your aliases".to_owned()]) },
        create_entity_into_registry : CreateEntityIntoRegistry { entity_registry_id :
        Some("your entity registry id".to_owned()), naming_strategy :
        Some("your naming strategy".to_owned()), folder_id : Some("your folder id"
        .to_owned()), registry_id : Some("your registry id".to_owned()) } }
    ];
    let response = client.bulk_create_mixtures(mixtures).send().await.unwrap();
    println!("{:#?}", response);
}
