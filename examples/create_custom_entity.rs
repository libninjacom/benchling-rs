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
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        author_ids: &["your author ids"],
        fields: Fields {},
        folder_id: "your folder id",
        registry_id: "your registry id",
    };
    let response = client.create_custom_entity(args).send().await.unwrap();
    println!("{:#?}", response);
}
