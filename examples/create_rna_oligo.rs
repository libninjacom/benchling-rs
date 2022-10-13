#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        fields: Fields {},
        folder_id: "your folder id",
        custom_fields: CustomFields {},
        registry_id: "your registry id",
        aliases: &["your aliases"],
        bases: "your bases",
        name: "your name",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
