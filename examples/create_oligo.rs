#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateOligoRequired {
        aliases: &["your aliases"],
        schema_id: "your schema id",
        registry_id: "your registry id",
        naming_strategy: "your naming strategy",
        author_ids: &["your author ids"],
        bases: "your bases",
        custom_fields: CustomFields {},
        folder_id: "your folder id",
        name: "your name",
        fields: Fields {},
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
