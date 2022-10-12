#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        naming_strategy: "your naming strategy",
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
        schema_id: "your schema id",
        bases: "your bases",
        aliases: &["your aliases"],
        fields: Fields {},
        name: "your name",
        folder_id: "your folder id",
        custom_fields: CustomFields {},
        registry_id: "your registry id",
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
