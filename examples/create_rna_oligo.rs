#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        registry_id: "your registry id",
        bases: "your bases",
        fields: Fields {},
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        name: "your name",
        naming_strategy: "your naming strategy",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
