#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::CreateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = CreateMoleculeRequired {
        name: "your name",
        entity_registry_id: "your entity registry id",
        naming_strategy: "your naming strategy",
        fields: Fields {},
        author_ids: &["your author ids"],
        registry_id: "your registry id",
        aliases: &["your aliases"],
        schema_id: "your schema id",
        folder_id: "your folder id",
        custom_fields: CustomFields {},
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
    };
    let response = client.create_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
