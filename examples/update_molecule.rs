#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateMoleculeRequired {
        molecule_id: "your molecule id",
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
        custom_fields: CustomFields {},
        fields: Fields {},
        folder_id: "your folder id",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        name: "your name",
        entity_registry_id: "your entity registry id",
        schema_id: "your schema id",
    };
    let response = client.update_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
