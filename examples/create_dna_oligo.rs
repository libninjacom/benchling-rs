#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        name: "your name",
        schema_id: "your schema id",
        fields: Fields {},
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        custom_fields: CustomFields {},
        registry_id: "your registry id",
        aliases: &["your aliases"],
        bases: "your bases",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
