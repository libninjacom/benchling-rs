#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        bases: "your bases",
        entity_registry_id: "your entity registry id",
        fields: Fields {},
        name: "your name",
        schema_id: "your schema id",
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        registry_id: "your registry id",
        aliases: &["your aliases"],
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
