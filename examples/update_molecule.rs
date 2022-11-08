#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateMoleculeRequired {
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        entity_registry_id: "your entity registry id",
        molecule_id: "your molecule id",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        name: "your name",
        aliases: &["your aliases"],
        fields: Fields {},
        chemical_structure: MoleculeStructure {
            structure_format: Some(::serde_json::json!({})),
            value: Some("your value".to_owned()),
        },
    };
    let response = client.update_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
