#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateOligoRequired {
        bases: "your bases",
        folder_id: "your folder id",
        registry_id: "your registry id",
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        name: "your name",
        fields: Fields {},
        aliases: &["your aliases"],
        schema_id: "your schema id",
        naming_strategy: "your naming strategy",
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
