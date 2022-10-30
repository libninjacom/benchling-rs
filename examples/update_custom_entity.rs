#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateCustomEntityRequired {
        aliases: &["your aliases"],
        name: "your name",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        custom_entity_id: "your custom entity id",
        fields: Fields {},
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
    };
    let response = client.update_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
