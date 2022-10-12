#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateCustomEntityRequired {
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        name: "your name",
        entity_registry_id: "your entity registry id",
        custom_entity_id: "your custom entity id",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        schema_id: "your schema id",
        fields: Fields {},
    };
    let response = client.update_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
