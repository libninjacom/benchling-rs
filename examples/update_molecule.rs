#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateMoleculeRequired {
        entity_registry_id: "your entity registry id",
        name: "your name",
        schema_id: "your schema id",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        chemical_structure: MoleculeStructure {
            value: Some("your value".to_owned()),
            structure_format: Some(::serde_json::json!({})),
        },
        custom_fields: CustomFields {},
        fields: Fields {},
        folder_id: "your folder id",
        molecule_id: "your molecule id",
    };
    let response = client.update_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
