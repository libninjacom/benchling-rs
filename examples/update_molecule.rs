#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateMoleculeRequired {
        fields: Fields {},
        molecule_id: "your molecule id",
        custom_fields: CustomFields {},
        name: "your name",
        schema_id: "your schema id",
        aliases: &["your aliases"],
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        entity_registry_id: "your entity registry id",
    };
    let response = client.update_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
