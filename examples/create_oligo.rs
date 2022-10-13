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
        naming_strategy: "your naming strategy",
        fields: Fields {},
        registry_id: "your registry id",
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
        name: "your name",
        custom_fields: CustomFields {},
        folder_id: "your folder id",
        bases: "your bases",
    };
    let response = client.create_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
