#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateOligoRequired {
        registry_id: "your registry id",
        aliases: &["your aliases"],
        fields: Fields {},
        schema_id: "your schema id",
        folder_id: "your folder id",
        naming_strategy: "your naming strategy",
        name: "your name",
        author_ids: &["your author ids"],
        bases: "your bases",
        entity_registry_id: "your entity registry id",
        custom_fields: CustomFields {},
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
