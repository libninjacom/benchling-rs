#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateCustomEntityRequired {
        name: "your name",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
        schema_id: "your schema id",
        fields: Fields {},
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        custom_fields: CustomFields {},
        folder_id: "your folder id",
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
