#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateCustomEntityRequired {
        name: "your name",
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
        registry_id: "your registry id",
        folder_id: "your folder id",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        aliases: &["your aliases"],
        naming_strategy: "your naming strategy",
        fields: Fields {},
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
