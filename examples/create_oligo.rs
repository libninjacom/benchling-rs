#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateOligoRequired {
        name: "your name",
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
        folder_id: "your folder id",
        bases: "your bases",
        aliases: &["your aliases"],
        fields: Fields {},
        registry_id: "your registry id",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        naming_strategy: "your naming strategy",
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
