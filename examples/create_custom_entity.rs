#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateCustomEntityRequired {
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        fields: Fields {},
        name: "your name",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
