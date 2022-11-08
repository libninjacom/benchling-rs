#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateCustomEntityRequired {
        author_ids: &["your author ids"],
        custom_entity_id: "your custom entity id",
        fields: Fields {},
        folder_id: "your folder id",
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        name: "your name",
        aliases: &["your aliases"],
        entity_registry_id: "your entity registry id",
    };
    let response = client.update_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
