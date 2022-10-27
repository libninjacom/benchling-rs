#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateRnaOligoRequired {
        aliases: &["your aliases"],
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        fields: Fields {},
        entity_registry_id: "your entity registry id",
        registry_id: "your registry id",
        bases: "your bases",
        folder_id: "your folder id",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        name: "your name",
    };
    let response = client.create_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
