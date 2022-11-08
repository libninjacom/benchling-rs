#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMoleculeRequired {
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        author_ids: &["your author ids"],
        registry_id: "your registry id",
        fields: Fields {},
        name: "your name",
        custom_fields: CustomFields {},
        chemical_structure: MoleculeStructure {
            structure_format: Some(::serde_json::json!({})),
            value: Some("your value".to_owned()),
        },
        schema_id: "your schema id",
        aliases: &["your aliases"],
        folder_id: "your folder id",
    };
    let response = client.create_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
