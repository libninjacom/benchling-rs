#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMoleculeRequired {
        fields: Fields {},
        custom_fields: CustomFields {},
        author_ids: &["your author ids"],
        name: "your name",
        aliases: &["your aliases"],
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
        folder_id: "your folder id",
        schema_id: "your schema id",
        naming_strategy: "your naming strategy",
        registry_id: "your registry id",
        entity_registry_id: "your entity registry id",
    };
    let response = client.create_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
