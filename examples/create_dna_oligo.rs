#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        name: "your name",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        bases: "your bases",
        fields: Fields {},
        entity_registry_id: "your entity registry id",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
