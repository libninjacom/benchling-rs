#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        schema_id: "your schema id",
        name: "your name",
        custom_fields: CustomFields {},
        fields: Fields {},
        aliases: &["your aliases"],
        bases: "your bases",
        registry_id: "your registry id",
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        folder_id: "your folder id",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
