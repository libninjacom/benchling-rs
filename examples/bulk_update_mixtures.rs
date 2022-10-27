#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_mixtures()
        .mixtures(
            vec![
                MixtureBulkUpdate { id : "your id".to_owned(), mixture_update :
                MixtureUpdate { fields : Some(Fields {}), name : Some("your name"
                .to_owned()), author_ids : Some(vec!["your author ids".to_owned()]),
                amount : Some("your amount".to_owned()), entity_registry_id :
                Some("your entity registry id".to_owned()), ingredients :
                Some(vec![IngredientWriteParams { catalog_identifier :
                Some("your catalog identifier".to_owned()), component_lot_text :
                Some("your component lot text".to_owned()), component_lot_container_id :
                Some("your component lot container id".to_owned()), notes :
                Some("your notes".to_owned()), units : Some("your units".to_owned()),
                component_lot_entity_id : Some("your component lot entity id"
                .to_owned()), amount : Some("your amount".to_owned()),
                component_entity_id : "your component entity id".to_owned() }]),
                custom_fields : Some(CustomFields {}), units : Some("your units"
                .to_owned()), schema_id : Some("your schema id".to_owned()), folder_id :
                Some("your folder id".to_owned()), aliases : Some(vec!["your aliases"
                .to_owned()]) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
