#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateOligoRequired {
        entity_registry_id: "your entity registry id",
        fields: Fields {},
        naming_strategy: "your naming strategy",
        bases: "your bases",
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        name: "your name",
        folder_id: "your folder id",
        registry_id: "your registry id",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
