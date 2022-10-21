#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMoleculeRequired {
        author_ids: &["your author ids"],
        naming_strategy: "your naming strategy",
        chemical_structure: MoleculeStructure {
            structure_format: Some(::serde_json::json!({})),
            value: Some("your value".to_owned()),
        },
        name: "your name",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        folder_id: "your folder id",
        entity_registry_id: "your entity registry id",
        fields: Fields {},
        custom_fields: CustomFields {},
        registry_id: "your registry id",
    };
    let response = client.create_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
