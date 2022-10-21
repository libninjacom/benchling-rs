#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        registry_id: "your registry id",
        name: "your name",
        author_ids: &["your author ids"],
        bases: "your bases",
        entity_registry_id: "your entity registry id",
        fields: Fields {},
        schema_id: "your schema id",
        naming_strategy: "your naming strategy",
        folder_id: "your folder id",
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
