#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMixtureRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMixtureRequired {
        aliases: &["your aliases"],
        amount: "your amount",
        fields: Fields {},
        custom_fields: CustomFields {},
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        ingredients: vec![
            IngredientWriteParams { units : Some("your units".to_owned()), amount :
            Some("your amount".to_owned()), component_lot_text :
            Some("your component lot text".to_owned()), notes : Some("your notes"
            .to_owned()), component_entity_id : "your component entity id".to_owned(),
            catalog_identifier : Some("your catalog identifier".to_owned()),
            component_lot_container_id : Some("your component lot container id"
            .to_owned()), component_lot_entity_id : Some("your component lot entity id"
            .to_owned()) }
        ],
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        name: "your name",
    };
    let response = client.create_mixture(args).units("your units").send().await.unwrap();
    println!("{:#?}", response);
}
