#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateCustomEntityRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateCustomEntityRequired {
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        registry_id: "your registry id",
        entity_registry_id: "your entity registry id",
        name: "your name",
        folder_id: "your folder id",
        aliases: &["your aliases"],
        fields: Fields {},
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
