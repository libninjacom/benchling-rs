#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        folder_id: "your folder id",
        fields: Fields {},
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        registry_id: "your registry id",
        name: "your name",
        bases: "your bases",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
