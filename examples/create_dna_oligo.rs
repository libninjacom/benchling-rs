#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateDnaOligoRequired {
        aliases: &["your aliases"],
        registry_id: "your registry id",
        bases: "your bases",
        naming_strategy: "your naming strategy",
        name: "your name",
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        fields: Fields {},
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
