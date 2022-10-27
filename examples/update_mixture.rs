#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let mixture_id = "your mixture id";
    let response = client
        .update_mixture(mixture_id)
        .aliases(&["your aliases"])
        .amount("your amount")
        .author_ids(&["your author ids"])
        .custom_fields(CustomFields {})
        .entity_registry_id("your entity registry id")
        .fields(Fields {})
        .folder_id("your folder id")
        .ingredients(
            vec![
                IngredientWriteParams { catalog_identifier :
                Some("your catalog identifier".to_owned()), component_lot_text :
                Some("your component lot text".to_owned()), component_lot_container_id :
                Some("your component lot container id".to_owned()), notes :
                Some("your notes".to_owned()), units : Some("your units".to_owned()),
                component_lot_entity_id : Some("your component lot entity id"
                .to_owned()), amount : Some("your amount".to_owned()),
                component_entity_id : "your component entity id".to_owned() }
            ],
        )
        .name("your name")
        .schema_id("your schema id")
        .units("your units")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
