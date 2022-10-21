#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateCustomEntityRequired {
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        custom_entity_id: "your custom entity id",
        aliases: &["your aliases"],
        name: "your name",
        entity_registry_id: "your entity registry id",
        fields: Fields {},
    };
    let response = client.update_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
