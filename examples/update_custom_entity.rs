#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateCustomEntityRequired {
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        fields: Fields {},
        entity_registry_id: "your entity registry id",
        name: "your name",
        folder_id: "your folder id",
        aliases: &["your aliases"],
        custom_entity_id: "your custom entity id",
        schema_id: "your schema id",
    };
    let response = client.update_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
