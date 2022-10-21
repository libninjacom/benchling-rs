#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateCustomEntityRequired {
        registry_id: "your registry id",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        fields: Fields {},
        folder_id: "your folder id",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        name: "your name",
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
