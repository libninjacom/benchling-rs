#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        author_ids: &["your author ids"],
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        aliases: &["your aliases"],
        name: "your name",
        bases: "your bases",
        registry_id: "your registry id",
        fields: Fields {},
        folder_id: "your folder id",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
