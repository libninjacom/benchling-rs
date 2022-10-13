#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMoleculeRequired {
        custom_fields: CustomFields {},
        naming_strategy: "your naming strategy",
        aliases: &["your aliases"],
        name: "your name",
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        fields: Fields {},
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
        schema_id: "your schema id",
        entity_registry_id: "your entity registry id",
        registry_id: "your registry id",
    };
    let response = client.create_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
